use dataloglite::{parse_datalog, parse_relation};

#[test]
fn test_parse_single_relation() {
    let input = r#"parent("Alice", "Bob")."#;
    let (remaining, relation) = parse_relation(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relation.relationship, "parent");
    assert_eq!(relation.parent, "Alice");
    assert_eq!(relation.child, "Bob");
}

#[test]
fn test_parse_multiple_relations() {
    let input = r#"parent("A", "B"). mother("B", "C")."#;
    let (remaining, relations) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 2);
    assert_eq!(relations[0].relationship, "parent");
    assert_eq!(relations[0].parent, "A");
    assert_eq!(relations[0].child, "B");
    assert_eq!(relations[1].relationship, "mother");
    assert_eq!(relations[1].parent, "B");
    assert_eq!(relations[1].child, "C");
}
