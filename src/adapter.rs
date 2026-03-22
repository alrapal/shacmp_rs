use std::{fmt::Display, str::FromStr};

use sha2::Digest;

/// Different type of SHA algorithm
/// supported.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HashType {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

/// Error type for invalid hash algorithm type.
#[derive(Debug, PartialEq, Eq)]
pub struct HashTypeError;

impl FromStr for HashType {
    /// Error in case the provided conversion from `&str` failed.
    type Err = HashTypeError;

    /// ## Brief:
    /// Allow to convert from a &str to a HashType.
    /// The conversions are the following:
    /// - Sha224: "224"
    /// - Sha256: "256"
    /// - Sha384: "384"
    /// - Sha512: "512"
    /// ## Parameters:
    /// - `s`: &str: The string to convert to `HashType`.
    /// ## Return:
    /// - `HashTypeError` if the string provided does not correspond
    /// to a valid string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "224" => Ok(HashType::Sha224),
            "256" => Ok(HashType::Sha256),
            "384" => Ok(HashType::Sha384),
            "512" => Ok(HashType::Sha512),
            _ => Err(HashTypeError),
        }
    }
}

impl Display for HashType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashType::Sha224 => write!(f, "Sha224"),
            HashType::Sha256 => write!(f, "Sha256"),
            HashType::Sha384 => write!(f, "Sha384"),
            HashType::Sha512 => write!(f, "Sha512"),
        }
    }
}

/// Wrapper for the different hash algorithm.
/// Provides a unified object and interface to create the hashes.
/// Uses external crates providing the hashing algorithms.
pub struct HasherWrapper<'a> {
    hash_type: &'a HashType,
}

impl<'a> HasherWrapper<'a> {
    /// ## Brief:
    /// Get a HasherWrapper instance configured to hash with the provided algorithm type.
    /// ## Parameters:
    /// - `HashType`: The type of hash algorithm the Hasher should use
    /// ## Return:
    /// - `HasherWrapper`: HasherWrapper instance
    pub fn new(hash_type: &'a HashType) -> Self {
        HasherWrapper { hash_type }
    }

    /// ## Brief:
    /// Create a hash string with on the provided content.
    /// The hashing algorithm is defined by the type used to create the `HasherWrapper`
    /// ## Parameters:
    /// - `content`: Array of &str: The content use to create a hash.
    /// ## Return:
    /// - A hex `String` representing the hash of the content
    pub fn hash(&self, content: &[u8]) -> String {
        let hash_type = self.hash_type;
        match *hash_type {
            HashType::Sha224 => {
                let tmp = sha2::Sha224::digest(content);
                // Format as a hex string
                format!("{:x}", tmp)
            }
            HashType::Sha256 => {
                let tmp = sha2::Sha256::digest(content);
                // Format as a hex string
                format!("{:x}", tmp)
            }
            HashType::Sha384 => {
                let tmp = sha2::Sha384::digest(content);
                // Format as a hex string
                format!("{:x}", tmp)
            }
            HashType::Sha512 => {
                let tmp = sha2::Sha512::digest(content);
                // Format as a hex string
                format!("{:x}", tmp)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod hash_type {
        use super::super::*;
        #[test]
        fn test_from_valid_strings() {
            let expected = HashType::Sha256;
            let result = HashType::from_str("256").unwrap();

            assert_eq!(expected, result);

            let expected = HashType::Sha512;
            let result = HashType::from_str("512").unwrap();

            assert_eq!(expected, result);

            let expected = HashType::Sha224;
            let result = HashType::from_str("224").unwrap();

            assert_eq!(expected, result);

            let expected = HashType::Sha384;
            let result = HashType::from_str("384").unwrap();

            assert_eq!(expected, result);
        }

        #[test]
        fn test_from_invalid_string() {
            let expected = HashTypeError;
            let result = HashType::from_str("25").expect_err("Should be invalid SHA type");

            assert_eq!(expected, result);
        }
    }
}
