use std::collections::HashSet;

use crate::parser::ConjunctiveQuery;

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

    // Checks if a relation exists in the database
    pub fn contains_relation(&self, relation: &crate::parser::Relation) -> bool {
        self.relations.contains(relation)
    }

    // Checks if a fact exists in the database
    pub fn contains_fact(&self, fact: &crate::parser::Fact) -> bool {
        self.facts.contains(fact)
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
