use std::env;
use std::process;

use shacmp_rs::comparator::Comparator;
use shacmp_rs::parser::Configuration;

const EXECUTABLE: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ## Brief:
/// Parse arguments from the CLI and if valid, tries to read the file and generate a hash value.
///
/// Compare the produced hash with a provided hex reference string and output on the standard output
/// the result of the comparison.
fn main() {
    print_header();
    // Build a configuration object based on the provided arguments.
    let arguments = Configuration::parse(env::args()).unwrap_or_else(|err| {
        // Print to std err if parsing failed.
        eprintln!("/!\\ Configuration Error: {}", err);
        print_help();
        process::exit(1);
    });

    // Print arguments for feedback to user.
    println!("{}\n", arguments);

    // Perform the comparison. Can fail if the path is incorrect.
    let comparator = Comparator::build(&arguments).unwrap_or_else(|err| {
        // Print to std err if reading file failed.
        eprintln!("/!\\ Application error: {}", err);
        print_help();
        process::exit(1);
    });

    // Print the result of the comparison.
    println!("Result:\t\t{:?}", comparator.compare());
}

/// ## Brief:
/// Print the name version and authors of the crate
fn print_header() {
    println!("------------------------------------------------------------------");
    println!(
        "{} - version {}\nAuthor(s): {}",
        EXECUTABLE, VERSION, AUTHORS
    );
    println!("------------------------------------------------------------------");
}

/// ## Brief:
/// Print usage instructions
fn print_help() {
    println!("------------------------------------------------------------------");
    println!(
        "\nUsage:\n\t{} <file path> <hash type> <reference hex string>\n",
        EXECUTABLE
    );
    println!("------------------------------------------------------------------");
}
