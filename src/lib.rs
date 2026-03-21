//! This crate is providing tools to generate a hash value for a file, and
//! compare it with a provided has reference string.
//! It supports different hashing algorithms of the SHA family, provided by the
//! `sha2` crate.
//!
//! ## Example
//! ```
//! use std::process;
//!
//! use shacmp_rs::comparator::Comparator;
//! use shacmp_rs::parser::Configuration;
//!
//! // Mocked CLI arguments for DocTest, should be replaced by `std::env::args()`
//! let mocked_arguments = vec![
//!     String::from("executable_path"),
//!     String::from("test.txt"),
//!     String::from("256"),
//!     String::from("c87e2ca771bab6024c269b933389d2a92d4941c848c52f155b9b84e1f109fe35")
//! ];
//!
//! let arguments = Configuration::parse(mocked_arguments.into_iter()).unwrap_or_else(|err| {
//!     eprintln!("Configuration Error: {}", err);
//!     process::exit(1);
//! });
//!
//! println!("{}\n", arguments);
//!
//! let comparator = Comparator::build(&arguments).unwrap_or_else(|err| {
//!     eprintln!("Application error: {}", err);
//!     process::exit(1);
//! });
//!
//! println!("Result:\t\t{:?}", comparator.run(&arguments));
//! ```

// Private module doing the relay between the public modules and the sha2 crate.
mod sha;

pub use sha::HashType;
pub mod comparator;
pub mod parser;
