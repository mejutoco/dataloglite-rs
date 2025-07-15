use crate::api::DatabaseInstance;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;
use crate::parser::NonQueryDatalogItem;
use std::io::Write;
use std::sync::Mutex;
use std::sync::OnceLock;

static DB_INSTANCE: OnceLock<Mutex<DatabaseInstance>> = OnceLock::new();

fn get_db_instance() -> &'static Mutex<DatabaseInstance> {
    DB_INSTANCE.get_or_init(|| Mutex::new(DatabaseInstance::new()))
}

pub fn execute_query<W: Write>(query: NonQueryDatalogItem, writer: &mut W) {
    let mut db = get_db_instance().lock().unwrap();
    let db = db.get_db_mut();
    match query {
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
    }
}

pub fn interpret<W: Write>(input: &str, writer: &mut W) {
    let mut db = get_db_instance().lock().unwrap();
    let db = db.get_db_mut();
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
                        // DatalogItem::Query(q) => execute_query(q.data, writer),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing datalog: {}", e),
    }
}
