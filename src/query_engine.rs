use crate::api::Database;
use crate::api::DatabaseInstance;
use crate::parser::parse_datalog;
use crate::parser::DatalogItem;
use crate::parser::NonQueryDatalogItem;
use crate::parser::QueryProjection;
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
        NonQueryDatalogItem::QueryProjectionFact(query) => {
            writeln!(writer, "Query: list all where {}(_)", query.name).unwrap();
            let results = db.query_projection_fact(query);
            writeln!(writer, "{}", results.iter().map(|r| r).format(", ")).unwrap();
        }
        NonQueryDatalogItem::QueryProjectionRelation(query) => {
            writeln!(
                writer,
                "Query: list all where {}({}, {})",
                query.name, query.first, query.second
            )
            .unwrap();
            let results = db.query_projection_relation(query);
            writeln!(writer, "{}", results.iter().map(|r| r).format(", ")).unwrap();
        }
        NonQueryDatalogItem::ConjunctiveQuery(query) => {
            // writeln!(writer, "Query: {}", query.data).unwrap();
            let mut text = String::new();
            for el in &query.data {
                match el {
                    QueryProjection::QueryProjectionFact(q) => {
                        // hardcode X as only possibility
                        // if we change the parser to allow more variables,
                        // we will need to change
                        text.push_str(&format!("\n    {}(X)", q.name));
                    }
                    QueryProjection::QueryProjectionRelation(q) => {
                        text.push_str(&format!("\n    {}({}, {})", q.name, q.first, q.second));
                    }
                }
            }
            writeln!(writer, "Query: list all where:{}", text).unwrap();
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

// @param reset_db: If true, clears the database before interpreting the input
pub fn interpret<W: Write>(input: &str, writer: &mut W, reset_db: Option<bool>) {
    let reset_db = reset_db.unwrap_or(false);
    let mut db_guard = get_db_instance().lock().unwrap();
    let db = db_guard.get_db_mut();
    if reset_db {
        db.clear();
        // Reinitialize the database instance by replacing it with a new one
        let _ = DB_INSTANCE.set(Mutex::new(DatabaseInstance::new()));
    }

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
