use dataloglite::parse_datalog;

use clap::Parser;
use std::fs; // Now this is the only Parser in scope

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

    if input.trim().is_empty() {
        eprintln!("Error: Input file is empty");
        std::process::exit(1);
    }

    match parse_datalog(&input) {
        Ok((_, items)) => {
            if items.is_empty() {
                println!("No valid datalog items found");
            } else {
                println!("Parsed items:");
                for rel in items {
                    match rel {
                        dataloglite::DatalogItem::Fact(rel) => {
                            println!("{} is {}", rel.name, rel.first)
                        }
                        dataloglite::DatalogItem::Relation(rel) => {
                            println!("{} is {} of {}", rel.name, rel.first, rel.second)
                        }
                        dataloglite::DatalogItem::Rule(rel) => {
                            println!("{} of {}, {} means TODO", rel.name, rel.first, rel.second)
                        }
                        dataloglite::DatalogItem::Query(q) => println!(
                            "Query: {} is {} of {}?",
                            q.relation.name, q.relation.first, q.relation.second
                        ),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing datalog: {}", e),
    }
}
