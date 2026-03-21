use std::{fmt::Display, str::FromStr};
/// Different type of SHA algorithm
/// supported.
#[derive(Debug, PartialEq, Eq)]
pub enum HashType {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

/// Error type for invalid SHA algorithm type
#[derive(Debug, PartialEq, Eq)]
pub struct ShaTypeError;

impl FromStr for HashType {
    type Err = ShaTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "224" => Ok(HashType::Sha224),
            "256" => Ok(HashType::Sha256),
            "384" => Ok(HashType::Sha384),
            "512" => Ok(HashType::Sha512),
            _ => Err(ShaTypeError),
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

#[cfg(test)]
mod tests {

    use super::*;

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
        let expected = ShaTypeError;
        let result = HashType::from_str("25").expect_err("Should be invalid SHA type");

        assert_eq!(expected, result);
    }
}
