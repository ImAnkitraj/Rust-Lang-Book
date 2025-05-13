use std::env;

use grep;
use std::process;

fn main() {

    let config = grep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
