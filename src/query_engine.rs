use crate::api::Database;
use crate::api::DatabaseInstance;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;
use crate::parser::NonQueryDatalogItem;
use crate::parser::VariableBasedRelation;
use itertools::Itertools;
use std::io::Write;
use std::sync::Mutex;
use std::sync::OnceLock;

static DB_INSTANCE: OnceLock<Mutex<DatabaseInstance>> = OnceLock::new();

fn get_db_instance() -> &'static Mutex<DatabaseInstance> {
    DB_INSTANCE.get_or_init(|| Mutex::new(DatabaseInstance::new()))
}

pub fn execute_query<W: Write>(query: NonQueryDatalogItem, db: &Database, writer: &mut W) {
    match query {
        NonQueryDatalogItem::QueryProjection(query) => {
            writeln!(
                writer,
                "Query: list all where {}({}, {})",
                query.name, query.first, query.second
            )
            .unwrap();
            let results = db.query_projection(query);
            writeln!(writer, "{}", results.iter().map(|r| r).format(", ")).unwrap();
        }
        NonQueryDatalogItem::ConjunctiveQuery(query) => {
            writeln!(writer, "Query: {}", query.name).unwrap();
            let results = db.query_conjunctive(query);
            writeln!(writer, "{}", results.iter().map(|r| r).format(", ")).unwrap();
        }
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
        NonQueryDatalogItem::VariableBasedRelation(item) => match item {
            VariableBasedRelation::VariableBasedRelationFirstIsVar(rel) => {
                writeln!(writer, "Query: Who is {} of {}?", rel.name, rel.second).unwrap();
                let relations = db.relations_where_second_is(&rel.name, &rel.second);
                writeln!(
                    writer,
                    "{}",
                    relations.iter().map(|r| &r.first).format(", ")
                )
                .unwrap();
            }
            VariableBasedRelation::VariableBasedRelationSecondIsVar(rel) => {
                writeln!(writer, "Query: Of whom is {} {}?", rel.first, rel.name).unwrap();
                let relations = db.relations_where_first_is(&rel.name, &rel.first);
                writeln!(
                    writer,
                    "{}",
                    relations.iter().map(|r| &r.second).format(", ")
                )
                .unwrap();
            }
        },
        _ => eprintln!("Unsupported query type"),
    }
}

pub fn interpret<W: Write>(input: &str, writer: &mut W) {
    let mut db_guard = get_db_instance().lock().unwrap();
    let db = db_guard.get_db_mut();

    match parse_datalog(input) {
        Ok((_, items)) => {
            if items.is_empty() {
                writeln!(writer, "No valid datalog items found").unwrap();
            } else {
                for item in items {
                    match item {
                        DatalogItem::Fact(fact) => {
                            writeln!(writer, "{} is {}", fact.name, fact.first).unwrap();
                            db.add_fact(fact);
                        }
                        DatalogItem::Relation(relation) => {
                            writeln!(
                                writer,
                                "{} is {} of {}",
                                relation.name, relation.first, relation.second
                            )
                            .unwrap();
                            db.add_relation(relation);
                        }
                        DatalogItem::Rule(rule) => {
                            writeln!(
                                writer,
                                "{} of {}, {} means TODO",
                                rule.name, rule.first, rule.second
                            )
                            .unwrap();
                        }
                        DatalogItem::Query(query) => {
                            // Use the already locked database instance
                            execute_query(query.data, db, writer);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing datalog: {}", e),
    }
}
