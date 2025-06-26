use dataloglite::{parse_datalog, ParentRelation};

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
    use super::*;

    #[test]
    fn test_parse_single_relation() {
        let input = r#"parent("Alice", "Bob")."#;
        let (remaining, relations) = parse_parent_relation(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(relations.parent, "Alice");
        assert_eq!(relations.child, "Bob");
    }

    #[test]
    fn test_parse_multiple_relations() {
        let input = r#"parent("A", "B"). parent("B", "C")."#;
        let (remaining, relations) = parse_datalog(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(relations.len(), 2);
        assert_eq!(relations[0].parent, "A");
        assert_eq!(relations[0].child, "B");
        assert_eq!(relations[1].parent, "B");
        assert_eq!(relations[1].child, "C");
    }
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
                println!("{} is parent of {}", rel.parent, rel.child);
            }
        }
        Err(e) => println!("Error parsing: {:?}", e),
    }
}
