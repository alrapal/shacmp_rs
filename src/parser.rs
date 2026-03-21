//! This module provides a CLI argument parser that provides a verified configuration
//! to provide to `crate::comparator::ShaComparator`.
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

use crate::sha::{HashType, ShaTypeError};

/// Different types of error that can occur during parsing
///
/// The different errors can be:
/// - `MissingFile`: The path to the file to produce a hash for comparison with the reference is not provided.
/// - `MissingShaType`: The type of SHA algorithm to use is not provided
/// - `InvalidShaType`: The type of SHA algorithm provided is not supported or does not exist
/// - `MissingShaRef`: The reference hash string is to compare with is not provided
#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    MissingFile,
    MissingShaType,
    InvalidShaType,
    MissingShaRef,
}

impl From<ShaTypeError> for ParsingError {
    fn from(_: ShaTypeError) -> Self {
        ParsingError::InvalidShaType
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFile => write!(f, "Missing file path"),
            Self::MissingShaRef => write!(f, "Missing SHA reference"),
            Self::MissingShaType => write!(f, "Missing SHA type"),
            Self::InvalidShaType => write!(f, "Invalid SHA type"),
        }
    }
}

/// Configuration resulting of the successful parsing of the CLI arguments.
/// This is used with the `shacmp_rs::ShaComparator` to produce a comparison.
#[derive(Debug, PartialEq, Eq)]
pub struct Configuration {
    pub sha_ref: String,
    pub sha_type: HashType,
    pub file_path: String,
}

impl Configuration {
    /// Parse the provided iterator into a `shacmp_rs::parser::Configuration` configuration object
    /// that can be used with the `shacmp_rs::comparator::ShaComparator`.
    ///
    /// This is expected to be used with the CLI, so the first item in the iterator is ignored since it is
    /// the path to the executable.
    ///
    /// ## Return
    /// - `Configuration` on success
    /// - `ParsingError` on failure
    ///
    /// ## Example
    /// ```
    /// use shacmp_rs::parser::{Configuration, ParsingError};
    ///
    /// let config: Result<Configuration, ParsingError> = Configuration::parse(std::env::args());
    /// ```
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Configuration, ParsingError> {
        // Ignore first argument that is the executable path
        args.next().unwrap();

        let file_path = args.next().ok_or(ParsingError::MissingFile)?;
        let sha_type = args.next().ok_or(ParsingError::MissingShaType)?;
        let sha_type = HashType::from_str(&sha_type)?;
        let sha_ref = args.next().ok_or(ParsingError::MissingShaRef)?;

        Ok(Configuration {
            sha_ref,
            sha_type,
            file_path,
        })
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File path:\t{}\nSHA type:\t{}\nSHA reference:\t{}",
            self.file_path, self.sha_type, self.sha_ref
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
            sha_type: HashType::Sha256,
            sha_ref: String::from(
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
        let expected = ParsingError::InvalidShaType;
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
        let expected = ParsingError::MissingShaType;
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
        let expected = ParsingError::MissingShaRef;
        // Assertion
        let result = Configuration::parse(input.into_iter())
            .expect_err("Should fail due to an missing sha type in argument list");

        assert_eq!(result, expected);
    }
}
