use std::env;
use std::process;

use shacmp_rs::comparator::Comparator;
use shacmp_rs::parser::Configuration;

const EXECUTABLE: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_header() {
    println!(
        "{} - version {}\nAuthor(s): {}",
        EXECUTABLE, VERSION, AUTHORS
    );
}

fn main() {
    print_header();
    // Build a configuration object based on the provided arguments.
    let arguments = Configuration::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("Configuration Error: {}", err);
        process::exit(1);
    });

    println!("{}\n", arguments);

    let comparator = Comparator::build(&arguments).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    println!("Result:\t\t{:?}", comparator.run(&arguments));
}
