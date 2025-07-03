use crate::api::Database;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;
use crate::parser::NonQueryDatalogItem;
use std::io::Write;

pub fn execute_query<W: Write>(input: &str, mut writer: W) {
    let mut db = Database::new();
    match parse_datalog(input) {
        Ok((_, items)) => {
            if items.is_empty() {
                writeln!(writer, "No valid datalog items found").unwrap();
            } else {
                // writeln!(writer, "Parsed items:").unwrap();
                for rel in items {
                    match rel {
                        DatalogItem::Fact(el) => {
                            writeln!(writer, "{} is {}", el.name, el.first).unwrap();
                            db.add_fact(el);
                        }
                        DatalogItem::Relation(el) => {
                            writeln!(writer, "{} is {} of {}", el.name, el.first, el.second)
                                .unwrap();
                            db.add_relation(el);
                        }
                        DatalogItem::Rule(el) => {
                            writeln!(
                                writer,
                                "{} of {}, {} means TODO",
                                el.name, el.first, el.second
                            )
                            .unwrap();
                            // add to sets
                        }
                        DatalogItem::Query(q) => match q.data {
                            NonQueryDatalogItem::Relation(rel) => {
                                writeln!(
                                    writer,
                                    "Query: {} is {} of {}?",
                                    rel.name, rel.first, rel.second
                                )
                                .unwrap();
                                if db.contains_relation(&rel) {
                                    writeln!(writer, "true").unwrap();
                                } else {
                                    writeln!(writer, "false").unwrap();
                                }
                            }
                            NonQueryDatalogItem::Fact(fact) => {
                                writeln!(writer, "Query: {} is {}", fact.name, fact.first).unwrap();
                                if db.contains_fact(&fact) {
                                    writeln!(writer, "true").unwrap();
                                } else {
                                    writeln!(writer, "false").unwrap();
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
