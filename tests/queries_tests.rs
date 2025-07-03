use dataloglite::parser::{parse_query, NonQueryDatalogItem};

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
