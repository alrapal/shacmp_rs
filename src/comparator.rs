//! Contains the comparison functionalities.
//! Requires a crate::parser::Configuration object to produce the comparison.
//!
//! ## Example
//! ```
//! use shacmp_rs::{
//!     algorithms::HashType,
//!     parser::Configuration,
//!     comparator::{
//!         Comparator,
//!         ComparisonResult
//!         }
//!     };
//!
//!    // Valid configuration object.
//!    // To generate configuration object, it is prefered to use the `Configuration::parse()` method.
//!    // as it will validate the inputs.
//!    let input = Configuration {
//!        file_path: String::from("Utils/test_file.txt"),
//!        hash_type: HashType::Sha224,
//!        hash_reference: String::from("7fd0d23a4d54951ac4c9065249ff29954eb911e57e911a65d7306274"),
//!    };
//!
//!    let comparator = Comparator::build(&input).expect("Should be valid inputs");
//!    let result = comparator.compare(); // The comparator is consumed here.
//!    assert_eq!(ComparisonResult::Equal, result);
//! ```
use std::error::Error;
use std::fs;

use crate::adapter::HasherWrapper;
use crate::parser::Configuration;

/// Possible results resulting of a comparison between
/// the produced hash hex string and the reference provided.
#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonResult {
    Equal,
    Different,
}

/// Retrieve the file content, produce a hash string for the file and compare it with the
/// reference string provided by the configuration
pub struct Comparator<'a> {
    /// Reference to the `Configuration` use for this `Comparator`.
    configuration: &'a Configuration,
    /// The content of the file provided by the `Configuration`.
    file_content: Vec<u8>,
}

impl<'a> Comparator<'a> {
    /// ## Brief:
    /// Build a new comparator based on the configuration. If the file cannot be read, will throw an error.
    /// ## Parameters:
    /// - `configuration`: `Configuration` object reference. Needs to outlive the Comparator.
    /// ## Return:
    /// - `Box<dyn Error>>` if reading the file failed
    /// - `Comparator` on success
    pub fn build(configuration: &'a Configuration) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read(&configuration.file_path)?;

        Ok(Comparator {
            configuration,
            file_content,
        })
    }

    /// ## Brief:
    /// After a successful `build()` call, compare the content of the file
    /// and the reference hex string provided by the `Configuration`.
    ///
    /// ***Note: Consumes the `Comparator`, making it unusable after this call.***
    /// ## Return:
    /// - `ComparisonResult::Equal` if the hashes are equal,
    /// - `ComparisonResult::Different` otherwise.
    pub fn compare(self) -> ComparisonResult {
        // Compute the hash using the algorithm from the configuration
        let hash = HasherWrapper::new(&self.configuration.hash_type).hash(&self.file_content);
        // Compare the reference string with the computed one
        match self.configuration.hash_reference == hash {
            true => ComparisonResult::Equal,
            false => ComparisonResult::Different,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter;

    const TEST_FILE: &str = "Utils/test_file.txt";
    // The following reference string have been produced using the series of shasum binaries from the GNU coreutils 9.7
    // On the test file located in the path above.
    const SHA224_REF: &str = "7fd0d23a4d54951ac4c9065249ff29954eb911e57e911a65d7306274";
    const SHA256_REF: &str = "c87e2ca771bab6024c269b933389d2a92d4941c848c52f155b9b84e1f109fe35";
    const SHA384_REF: &str = "1600a408df6f0775d5d3d2f13d8355a7a668ffc1be13810041e883f510b05dba0662a55c0b6b9a49c51293fa892d00d7";
    const SHA512_REF: &str = "3de78a913cb8896f8f08ce3374b726b49ed00cc569621c5161c31eb80fca4d2f5e4443d42676dfc79743f345de7f0b95dbb2c97b2bc1a438a5a49c5f1b5298ac";

    #[test]
    fn test_compared_sha224_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha224,
            hash_reference: String::from(SHA224_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha224_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha224,
            // Change the reference string to simulate the unmatch
            hash_reference: String::from(SHA224_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha256_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha256,
            hash_reference: String::from(SHA256_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha256_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha256,
            // Change the reference string to simulate the unmatch
            hash_reference: String::from(SHA256_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha384_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha384,
            hash_reference: String::from(SHA384_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha384_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha384,
            // Change the reference string to simulate the unmatch
            hash_reference: String::from(SHA384_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha512_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha512,
            hash_reference: String::from(SHA512_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha512_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            hash_type: adapter::HashType::Sha512,
            // Change the reference string to simulate the unmatch
            hash_reference: String::from(SHA512_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.compare();
        assert_eq!(expected, result);
    }
}
