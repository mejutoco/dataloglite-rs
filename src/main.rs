use dataloglite::parser::parse_datalog;
use dataloglite::parser::DatalogItem;
use dataloglite::query_engine::execute_query;
use std::collections::HashSet;

use clap::Parser;
use std::fs;

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

    if input.trim().is_empty() {
        eprintln!("Error: Input file is empty");
        std::process::exit(1);
    }

    // let mut facts: HashSet<dataloglite::parser::Fact> = HashSet::new();
    // let mut relations: HashSet<dataloglite::parser::Relation> = HashSet::new();

    execute_query(&input);
}
