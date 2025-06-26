use dataloglite::parse_datalog;

use std::{fs};
use clap::Parser;  // Now this is the only Parser in scope

/// Simple program to parse datalog files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to parse
    input_file: String,
}

#[cfg(test)]
mod tests {
    // Empty test module kept for consistency
}

fn main() {
    let args = Args::parse();

    let input = match fs::read_to_string(&args.input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    match parse_datalog(&input) {
        Ok((_, relations)) => {
            println!("Parsed relations:");
            for rel in relations {
                println!("{} is parent of {}", rel.first, rel.second);
            }
        }
        Err(e) => println!("Error parsing: {:?}", e),
    }
}
