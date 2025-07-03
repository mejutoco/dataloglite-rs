use crate::api::Database;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;
use crate::parser::NonQueryDatalogItem;

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
                        DatalogItem::Query(q) => match q.data {
                            NonQueryDatalogItem::Relation(rel) => {
                                println!("Query: {} is {} of {}?", rel.name, rel.first, rel.second);
                                if db.contains_relation(&rel) {
                                    println!("true");
                                } else {
                                    println!("false");
                                }
                            }
                            NonQueryDatalogItem::Fact(fact) => {
                                println!("Query: {} is {}", fact.name, fact.first);
                                if db.contains_fact(&fact) {
                                    println!("true");
                                } else {
                                    println!("false");
                                }
                            }
                            _ => eprintln!("Unsupported query type"),
                        },
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing datalog: {}", e),
    }
}
