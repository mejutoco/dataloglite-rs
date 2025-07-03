use crate::api::Database;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;

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
                        }
                        DatalogItem::Relation(el) => {
                            println!("{} is {} of {}", el.name, el.first, el.second);
                            db.add_relation(el);
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
