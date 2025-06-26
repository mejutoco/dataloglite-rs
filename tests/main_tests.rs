use dataloglite::{parse_datalog, parse_relation};

#[test]
fn test_parse_single_relation() {
    let input = r#"parent("Alice", "Bob")."#;
    let (remaining, relation) = parse_relation(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relation.relationship, "parent");
    assert_eq!(relation.first, "Alice");
    assert_eq!(relation.second, "Bob");
}

#[test]
fn test_parse_multiple_relations() {
    let input = r#"parent("A", "B"). mother("B", "C")."#;
    let (remaining, relations) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 2);
    assert_eq!(relations[0].relationship, "parent");
    assert_eq!(relations[0].first, "A");
    assert_eq!(relations[0].second, "B");
    assert_eq!(relations[1].relationship, "mother");
    assert_eq!(relations[1].first, "B");
    assert_eq!(relations[1].second, "C");
}

#[test]
fn test_parse_fact() {
    let input r#"male(X)."#;
    let (remaining, fact) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(fact.name, "male");
    assert_eq!(fact.first, "X");
}

#[test]
fn test_parse_rule() {
    let input r#"father(X, Y) :- parent(X, Y), male(X)."#;
    let (remaining, rule) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(rule.name, "father");
    assert_eq!(rule.first, "X");
    assert_eq!(rule.second, "Y");
    assert_eq!(rule.relations.len(), 2);
    assert_eq!(rule.relations[0].relationship, "parent");
    assert_eq!(rule.relations[0].first, "X");
    assert_eq!(rule.relations[0].second, "Y");
}
