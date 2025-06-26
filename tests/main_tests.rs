use dataloglite::{parse_datalog, parse_parent_relation};

#[test]
fn test_parse_single_relation() {
    let input = r#"parent("Alice", "Bob")."#;
    let (remaining, relations) = parse_parent_relation(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relations.parent, "Alice");
    assert_eq!(relations.child, "Bob");
}

#[test]
fn test_parse_multiple_relations() {
    let input = r#"parent("A", "B"). parent("B", "C")."#;
    let (remaining, relations) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 2);
    assert_eq!(relations[0].parent, "A");
    assert_eq!(relations[0].child, "B");
    assert_eq!(relations[1].parent, "B");
    assert_eq!(relations[1].child, "C");
}
