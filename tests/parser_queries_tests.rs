use dataloglite::parser::{parse_query, NonQueryDatalogItem, VariableBasedRelation};

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

// #[test]
// fn test_parse_query_variable_based_relation_first_is_var() {
//     let input = r#"?parent("Alice", "X")."#;
//     let (remaining, query) = parse_query(input).unwrap();
//     let NonQueryDatalogItem::Relation(el) = query.data else {
//         panic!("Expected NonQueryDatalogItem::Relation");
//     };
//     assert_eq!(remaining, "");
//     assert_eq!(el.name, "parent");
//     assert_eq!(el.first, "Alice");
//     assert_eq!(el.second, "Bob");
// }

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
