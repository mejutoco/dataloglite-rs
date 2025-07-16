use dataloglite::parser::{
    parse_query, ConjunctiveQueryDefinition, NonQueryDatalogItem, VariableBasedRelation,
};

#[test]
fn test_parse_query_relation() {
    let input = r#"?parent("Alice", "Bob")."#;
    let (remaining, query) = parse_query(input).unwrap();
    let NonQueryDatalogItem::Relation(el) = query.data else {
        panic!("Expected NonQueryDatalogItem::Relation");
    };
    assert_eq!(remaining, "");
    assert_eq!(el.name, "parent");
    assert_eq!(el.first, "Alice");
    assert_eq!(el.second, "Bob");
}

#[test]
fn test_parse_query_variable_based_relation_first_is_var() {
    let input = r#"?parent("X", "Bob")."#;
    let (remaining, query) = parse_query(input).unwrap();
    let NonQueryDatalogItem::VariableBasedRelation(vbr) = query.data else {
        panic!("Expected NonQueryDatalogItem::VariableBasedRelation");
    };

    let VariableBasedRelation::VariableBasedRelationFirstIsVar(el) = vbr else {
        panic!("Expected VariableBasedRelationFirstIsVar variant");
    };

    assert_eq!(remaining, "");
    assert_eq!(el.name, "parent");
    assert_eq!(el.second, "Bob");
}

#[test]
fn test_parse_query_variable_based_relation_second_is_var() {
    let input = r#"?parent("Alice", "X")."#;
    let (remaining, query) = parse_query(input).unwrap();
    let NonQueryDatalogItem::VariableBasedRelation(vbr) = query.data else {
        panic!("Expected NonQueryDatalogItem::VariableBasedRelation");
    };

    let VariableBasedRelation::VariableBasedRelationSecondIsVar(el) = vbr else {
        panic!("Expected VariableBasedRelationSecondIsVar variant");
    };

    assert_eq!(remaining, "");
    assert_eq!(el.name, "parent");
    assert_eq!(el.first, "Alice");
}

#[test]
fn test_parse_query_projection_relation_first_unknown() {
    let input = r#"?parent(_, Y)."#;
    let (remaining, query) = parse_query(input).unwrap();
    print!("Parsed query: {:#?}", query);
    let NonQueryDatalogItem::QueryProjectionRelation(q) = query.data else {
        panic!("Expected NonQueryDatalogItem::QueryProjectionRelation");
    };

    assert_eq!(remaining, "");
    assert_eq!(q.name, "parent");
    assert_eq!(q.first, "_");
    assert_eq!(q.second, "Y");
}

#[test]
fn test_parse_query_projection_relation_second_unknown() {
    let input = r#"?parent(X, _)."#;
    let (remaining, query) = parse_query(input).unwrap();
    print!("Parsed query: {:#?}", query);
    let NonQueryDatalogItem::QueryProjectionRelation(q) = query.data else {
        panic!("Expected NonQueryDatalogItem::QueryProjectionRelation");
    };

    assert_eq!(remaining, "");
    assert_eq!(q.name, "parent");
    assert_eq!(q.first, "X");
    assert_eq!(q.second, "_");
}

#[test]
fn test_parse_query_projection_fact() {
    let input = r#"?male(_)."#;
    let (remaining, query) = parse_query(input).unwrap();
    print!("Parsed query: {:#?}", query);
    let NonQueryDatalogItem::QueryProjectionFact(q) = query.data else {
        panic!("Expected NonQueryDatalogItem::QueryProjectionFact");
    };

    assert_eq!(remaining, "");
    assert_eq!(q.name, "male");
}

#[test]
fn test_parse_query_fact() {
    let input = r#"?female("Alice")."#;
    let (remaining, query) = parse_query(input).unwrap();
    let NonQueryDatalogItem::Fact(el) = query.data else {
        panic!("Expected NonQueryDatalogItem::Fact");
    };
    assert_eq!(remaining, "");
    assert_eq!(el.name, "female");
    assert_eq!(el.first, "Alice");
}

#[test]
fn test_parse_conjunctive_query() {
    let input = r#"?father(X, Y) :- parent(X, Y), male(X)."#;
    let (remaining, query) = parse_query(input).unwrap();
    assert_eq!(remaining, "");

    let NonQueryDatalogItem::ConjunctiveQuery(el) = query.data else {
        panic!("Expected ConjunctiveQuery variant");
    };

    assert_eq!(el.name, "father");
    assert_eq!(el.first, "X");
    assert_eq!(el.second, "Y");
    assert_eq!(el.definition.data.len(), 2);

    let ConjunctiveQueryDefinition { data } = &el.definition;
    assert_eq!(data.len(), 2);

    // Check first relation
    let NonQueryDatalogItem::Relation(rel) = &data[0] else {
        panic!("Expected Relation");
    };
    assert_eq!(rel.name, "parent");
    assert_eq!(rel.first, "X");
    assert_eq!(rel.second, "Y");

    // Check fact
    let NonQueryDatalogItem::Fact(rel) = &data[1] else {
        panic!("Expected Fact");
    };
    assert_eq!(rel.name, "male");
    assert_eq!(rel.first, "X");
}
