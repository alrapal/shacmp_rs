//! This module contains the tools to perform the comparison.
use sha2::Digest;

use crate::sha::HashType;

use super::parser::Configuration;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonResult {
    Equal,
    Different,
}

pub struct Comparator {
    file_content: String,
}

impl Comparator {
    pub fn build(config: &Configuration) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(&config.file_path)?;

        Ok(Comparator { file_content })
    }

    pub fn run(self, config: &Configuration) -> ComparisonResult {
        // Compute the hash using the corresponding algorithm
        let hash = match config.sha_type {
            HashType::Sha224 => {
                let tmp = sha2::Sha224::digest(self.file_content.as_bytes());
                // Format as a hex string
                format!("{:x}", tmp)
            }
            HashType::Sha256 => {
                let tmp = sha2::Sha256::digest(self.file_content.as_bytes());
                // Format as a hex string
                format!("{:x}", tmp)
            }
            HashType::Sha384 => {
                let tmp = sha2::Sha384::digest(self.file_content.as_bytes());
                // Format as a hex string
                format!("{:x}", tmp)
            }
            HashType::Sha512 => {
                let tmp = sha2::Sha512::digest(self.file_content.as_bytes());
                // Format as a hex string
                format!("{:x}", tmp)
            }
        };

        // Compare the reference string with the computed one
        match config.sha_ref == hash {
            true => ComparisonResult::Equal,
            false => ComparisonResult::Different,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha;

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
            sha_type: sha::HashType::Sha224,
            sha_ref: String::from(SHA224_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha224_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha224,
            // Change the reference string to simulate the unmatch
            sha_ref: String::from(SHA224_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha256_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha256,
            sha_ref: String::from(SHA256_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha256_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha256,
            // Change the reference string to simulate the unmatch
            sha_ref: String::from(SHA256_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha384_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha384,
            sha_ref: String::from(SHA384_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha384_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha384,
            // Change the reference string to simulate the unmatch
            sha_ref: String::from(SHA384_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha512_are_equal() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha512,
            sha_ref: String::from(SHA512_REF),
        };

        let expected = ComparisonResult::Equal;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compared_sha512_are_different() {
        // Expected Arguments
        let input = Configuration {
            file_path: String::from(TEST_FILE),
            sha_type: sha::HashType::Sha512,
            // Change the reference string to simulate the unmatch
            sha_ref: String::from(SHA512_REF).to_ascii_uppercase(),
        };

        let expected = ComparisonResult::Different;

        let comparator = Comparator::build(&input).expect("Should be valid inputs");
        let result = comparator.run(&input);
        assert_eq!(expected, result);
    }
}
