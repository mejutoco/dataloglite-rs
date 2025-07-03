use crate::api::Database;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;
use std::collections::HashSet;

// let mut facts: HashSet<crate::parser::Fact> = HashSet::new();
// let mut relations: HashSet<crate::parser::Relation> = HashSet::new();

pub fn execute_query(input: &str) {
    let mut db = Database::new();
    match parse_datalog(input) {
        Ok((_, items)) => {
            if items.is_empty() {
                println!("No valid datalog items found");
            } else {
                println!("Parsed items:");
                for rel in items {
                    match rel {
                        DatalogItem::Fact(el) => {
                            println!("{} is {}", el.name, el.first);
                            db.add_fact(el);
                            // add to sets
                            // facts.insert(el);
                        }
                        DatalogItem::Relation(el) => {
                            println!("{} is {} of {}", el.name, el.first, el.second);
                            db.add_relation(el);
                            // add to sets
                            // relations.insert(el);
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
                            // if relations.contains(&q.relation) {
                            if db.contains_relation(&q.relation) {
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
