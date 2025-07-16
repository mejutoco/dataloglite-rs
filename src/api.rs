use std::collections::HashSet;

use crate::parser::{ConjunctiveQuery, QueryProjectionFact, QueryProjectionRelation};

pub struct Database {
    facts: HashSet<crate::parser::Fact>,
    relations: HashSet<crate::parser::Relation>,
}

pub struct DatabaseInstance {
    db: Database,
}

impl DatabaseInstance {
    /// Creates a new, empty Database instance
    pub fn new() -> Self {
        DatabaseInstance {
            db: Database::new(),
        }
    }

    /// Gets a reference to the underlying database
    pub fn get_db(&self) -> &Database {
        &self.db
    }

    /// Gets a mutable reference to the underlying database
    pub fn get_db_mut(&mut self) -> &mut Database {
        &mut self.db
    }
}

impl Database {
    /// Creates a new, empty Database
    pub fn new() -> Self {
        Database {
            facts: HashSet::new(),
            relations: HashSet::new(),
        }
    }

    /// Adds facts to the database
    pub fn add_facts(&mut self, facts: impl IntoIterator<Item = crate::parser::Fact>) {
        self.facts.extend(facts);
    }

    /// Adds a single fact to the database
    pub fn add_fact(&mut self, fact: crate::parser::Fact) {
        self.facts.insert(fact);
    }

    /// Adds relations to the database
    pub fn add_relations(&mut self, relations: impl IntoIterator<Item = crate::parser::Relation>) {
        self.relations.extend(relations);
    }

    /// Adds a single relation to the database
    pub fn add_relation(&mut self, relation: crate::parser::Relation) {
        self.relations.insert(relation);
    }

    /// Gets a reference to the facts
    pub fn facts(&self) -> &HashSet<crate::parser::Fact> {
        &self.facts
    }

    /// Gets a reference to the relations
    pub fn relations(&self) -> &HashSet<crate::parser::Relation> {
        &self.relations
    }

    // Clears the database
    pub fn clear(&mut self) {
        self.facts.clear();
        self.relations.clear();
    }

    // Checks if a relation exists in the database
    pub fn contains_relation(&self, relation: &crate::parser::Relation) -> bool {
        self.relations.contains(relation)
    }

    // Checks if a fact exists in the database
    pub fn contains_fact(&self, fact: &crate::parser::Fact) -> bool {
        self.facts.contains(fact)
    }

    pub fn query_projection_relation(&self, q: QueryProjectionRelation) -> Vec<String> {
        // print!("Parsed items: {:#?}", q);
        // print!("all relations: {:#?}", self.relations);
        let mut results = HashSet::new();
        match (q.first.as_str(), q.second.as_str()) {
            // if first is a variable, we return all second
            ("_", _second) => {
                for relation in &self.relations {
                    if relation.name == q.name {
                        results.insert(relation.first.clone());
                    }
                }
            }
            // if second is a variable, we return all first
            (_first, "_") => {
                for relation in &self.relations {
                    if relation.name == q.name {
                        results.insert(relation.second.clone());
                    }
                }
            }
            _ => unimplemented!("Query projection for non-variable cases not implemented"),
        }
        let mut results_vec: Vec<String> = results.into_iter().collect();
        results_vec.sort();
        results_vec
    }

    pub fn query_projection_fact(&self, q: QueryProjectionFact) -> Vec<String> {
        // print!("Parsed items: {:#?}", q);
        // print!("all relations: {:#?}", self.relations);
        let mut results = HashSet::new();
        for fact in &self.facts {
            if fact.name == q.name {
                results.insert(fact.first.clone());
            }
        }
        let mut results_vec: Vec<String> = results.into_iter().collect();
        results_vec.sort();
        results_vec
    }

    // And query
    pub fn query_conjunctive(&self, q: ConjunctiveQuery) -> Vec<String> {
        print!("Parsed items: {:#?}", q);
        let mut results = Vec::new();
        // we keep here results that might match
        // and progressively filter them with each condition
        // ?parent(X, Y), male(X).
        let mut maybe_matches = HashSet::new();
        for item in q.definition.data {
            match item {
                crate::parser::NonQueryDatalogItem::Relation(rel) => {
                    if self.contains_relation(&rel) {
                        maybe_matches.insert((rel.first.clone(), rel.second.clone()));
                    }
                }
                crate::parser::NonQueryDatalogItem::Fact(fact) => {
                    if self.contains_fact(&fact) {
                        results.push(fact.first);
                    }
                }
                _ => continue, // Skip unsupported items
            }
        }
        // // Sort alphabetically by the 'second' field of the relation
        // results.sort_by(|a, b| a.second.cmp(&b.second));
        return results;
    }

    // TODO: relations_or_rule_where_first_is
    // we could either precalculate all the rules or calculate them on the fly
    // probably we precalculate all rules and iterate over which ones are relevant for this query
    pub fn relations_where_first_is(
        &self,
        rel_name: &str,
        first: &str,
    ) -> Vec<&crate::parser::Relation> {
        let mut results = Vec::new();
        for relation in &self.relations {
            if relation.name == rel_name && relation.first == first {
                results.push(relation);
            }
        }
        // Sort alphabetically by the 'second' field of the relation
        results.sort_by(|a, b| a.second.cmp(&b.second));
        return results;
    }

    // TODO: extract common between first and second
    pub fn relations_where_second_is(
        &self,
        rel_name: &str,
        second: &str,
    ) -> Vec<&crate::parser::Relation> {
        let mut results = Vec::new();
        for relation in &self.relations {
            if relation.name == rel_name && relation.second == second {
                results.push(relation);
            }
        }
        // Sort alphabetically by the 'first' field of the relation
        results.sort_by(|a, b| a.first.cmp(&b.first));
        return results;
    }

    // TODO: add query for any query as string
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
