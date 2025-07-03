use std::collections::HashSet;

pub struct Database {
    facts: HashSet<crate::parser::Fact>,
    relations: HashSet<crate::parser::Relation>,
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
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
