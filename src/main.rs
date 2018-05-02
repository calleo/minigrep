extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing command line arguments: {}", error);
        process::exit(1);
    });
    if let Err(error) = minigrep::run(config) {
        eprintln!("Problem processing file: {}", error);
        process::exit(1);
    }
}

