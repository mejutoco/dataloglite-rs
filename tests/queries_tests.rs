use dataloglite::parser::parse_query;

#[test]
fn test_parse_query_relation() {
    let input = r#"?parent("Alice", "Bob")."#;
    let (remaining, query) = parse_query(input).unwrap();
    let data = query.data;
    assert_eq!(remaining, "");
    assert_eq!(data.name, "parent");
    assert_eq!(data.first, "Alice");
    assert_eq!(data.second, "Bob");
}

#[test]
fn test_parse_query_fact() {
    let input = r#"?female("Alice")."#;
    let (remaining, query) = parse_query(input).unwrap();
    let data = query.data;
    assert_eq!(remaining, "");
    assert_eq!(data.name, "parent");
    assert_eq!(data.first, "Alice");
    assert_eq!(data.second, "Bob");
}

// #[test]
// fn test_query_relation_is_true() {
//     let input = r#"parent("Alice", "Bob")."#;
//     let (remaining, relation) = parse_relation(input).unwrap();
//     assert_eq!(remaining, "");
//     assert_eq!(relation.name, "parent");
//     assert_eq!(relation.first, "Alice");
//     assert_eq!(relation.second, "Bob");
// }
