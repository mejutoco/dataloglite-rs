use dataloglite::parse_datalog;
use dataloglite::DatalogItem;
use std::collections::HashSet;

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

    let mut facts: HashSet<dataloglite::Fact> = HashSet::new();
    let mut relations: HashSet<dataloglite::Relation> = HashSet::new();

    match parse_datalog(&input) {
        Ok((_, items)) => {
            if items.is_empty() {
                println!("No valid datalog items found");
            } else {
                println!("Parsed items:");
                for rel in items {
                    match rel {
                        DatalogItem::Fact(el) => {
                            println!("{} is {}", el.name, el.first);
                            // add to sets
                            facts.insert(el);
                        }
                        DatalogItem::Relation(el) => {
                            println!("{} is {} of {}", el.name, el.first, el.second);
                            // add to sets
                            relations.insert(el);
                        }
                        DatalogItem::Rule(el) => {
                            println!("{} of {}, {} means TODO", el.name, el.first, el.second);
                            // add to sets
                        }
                        DatalogItem::Query(q) => {
                            println!(
                                "Query: {} is {} of {}?",
                                q.relation.name, q.relation.first, q.relation.second
                            );
                            // execute query
                            if relations.contains(&q.relation) {
                                println!("true");
                            } else {
                                println!("false");
                            }
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing datalog: {}", e),
    }
}
