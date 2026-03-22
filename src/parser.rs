//! This module provides a CLI argument parser that provides a verified configuration
//! to provide to `crate::comparator::Comparator`.
//!
//! # Example
//!
//! ```
//! use shacmp_rs::parser::{Configuration, ParsingError};
//!
//! let config: Result<Configuration, ParsingError> = Configuration::parse(std::env::args());
//! ```

use std::{
    fmt::{self},
    str::FromStr,
};

use crate::adapter::{HashType, HashTypeError};

/// Different types of error that can occur during parsing
///
/// The different errors can be:
/// - `MissingFile`   : The path to the file to produce a hash for comparison with the reference is not provided.
/// - `MissingShaType`: The type of SHA algorithm to use is not provided
/// - `InvalidShaType`: The type of SHA algorithm provided is not supported or does not exist
/// - `MissingShaRef` : The reference hash string is to compare with is not provided
#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    MissingFile,
    MissingHashType,
    InvalidHashType,
    MissingHashReference,
}

impl From<HashTypeError> for ParsingError {
    fn from(_: HashTypeError) -> Self {
        ParsingError::InvalidHashType
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFile => write!(f, "Missing path to file to process"),
            Self::MissingHashReference => write!(f, "Missing hash reference for comparison"),
            Self::MissingHashType => write!(f, "Missing hash algorithm type"),
            Self::InvalidHashType => write!(f, "Invalid hash algorithm type"),
        }
    }
}

/// Configuration resulting of the successful parsing of the CLI arguments.
/// This is used with the `shacmp_rs::ShaComparator` to produce a comparison.
#[derive(Debug, PartialEq, Eq)]
pub struct Configuration {
    pub hash_reference: String,
    pub hash_type: HashType,
    pub file_path: String,
}

impl Configuration {
    /// ## Brief:
    /// Parse the provided iterator into a `shacmp_rs::parser::Configuration` configuration object
    /// that can be used with the `shacmp_rs::comparator::ShaComparator`.
    ///
    /// This is expected to be used with the CLI, so the first item in the iterator is ignored since it is
    /// the path to the executable.
    /// ## Parameters:
    /// - An `Iterator` over `String` items
    ///   - *Note: The order of the items should be the following:*
    ///     - 1: Path to executable (ignored during parsing)
    ///     - 2: Path to the file to process
    ///     - 3: Type of hash algorithm to use (See `HashType::from_str()` for valid input).
    ///     - 4: Hex string of the reference hash to compare with.
    /// ## Return:
    /// - `Configuration` on success
    /// - `ParsingError` on failure
    /// ## Example:
    /// ```
    /// use shacmp_rs::parser::{Configuration, ParsingError};
    ///
    /// let config: Result<Configuration, ParsingError> = Configuration::parse(std::env::args());
    /// ```
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Configuration, ParsingError> {
        // Ignore first argument that is the executable path
        args.next().unwrap();

        let file_path = args.next().ok_or(ParsingError::MissingFile)?;
        let hash_type = args.next().ok_or(ParsingError::MissingHashType)?;
        let hash_type = HashType::from_str(&hash_type)?;
        let hash_reference = args.next().ok_or(ParsingError::MissingHashReference)?;

        Ok(Configuration {
            hash_reference,
            hash_type,
            file_path,
        })
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Path to file to process:\t\t{}\nSelected hash algorithm type:\t\t{}\nHash string reference to compare:\t{}",
            self.file_path, self.hash_type, self.hash_reference
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_arguments() {
        // Simulated inputs
        let input = [
            String::from("Executable Path - Ignored"),
            String::from("test.txt"),
            String::from("256"),
            String::from("c87e2ca771bab6024c269b933389d2a92d4941c848c52f155b9b84e1f109fe35"),
        ];

        // Expected Argument
        let expected = Configuration {
            file_path: String::from("test.txt"),
            hash_type: HashType::Sha256,
            hash_reference: String::from(
                "c87e2ca771bab6024c269b933389d2a92d4941c848c52f155b9b84e1f109fe35",
            ),
        };

        // Assertion
        let result =
            Configuration::parse(input.into_iter()).expect("Should be valid argument list");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_invalid_sha_type() {
        let input = [
            String::from("Executable Path - Ignored"),
            String::from("test.txt"),
            String::from("invalid"),
            String::from("c87e2ca771bab6024c269b933389d2a92d4941c848c52f155b9b84e1f109fe35"),
        ];
        let expected = ParsingError::InvalidHashType;
        // Assertion
        let result = Configuration::parse(input.into_iter())
            .expect_err("Should fail due to an Invalid ShaType");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_missing_file_path() {
        // Call with empty arguments. First to fail is the file path
        let input = [String::from("Executable Path - Ignored")];
        let expected = ParsingError::MissingFile;
        // Assertion
        let result = Configuration::parse(input.into_iter())
            .expect_err("Should fail due to an missing file in argument list");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_missing_sha_type() {
        // Call partial valid arguments. Second to fail is the sha type.
        let input = [
            String::from("Executable Path - Ignored"),
            String::from("test.txt"),
        ];
        let expected = ParsingError::MissingHashType;
        // Assertion
        let result = Configuration::parse(input.into_iter())
            .expect_err("Should fail due to an missing sha type in argument list");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_missing_sha_reference() {
        // Call with partial valid arguments. Third to fail is the sha reference.
        let input = [
            String::from("Executable Path - Ignored"),
            String::from("test.txt"),
            String::from("256"),
        ];
        let expected = ParsingError::MissingHashReference;
        // Assertion
        let result = Configuration::parse(input.into_iter())
            .expect_err("Should fail due to an missing sha type in argument list");

        assert_eq!(result, expected);
    }
}
