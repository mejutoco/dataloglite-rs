use dataloglite::{parse_datalog, parse_relation, parse_rule_definition};
use dataloglite::{DatalogItem, RuleDefinition};

#[test]
fn test_parse_single_relation() {
    let input = r#"parent("Alice", "Bob")."#;
    let (remaining, relation) = parse_relation(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relation.name, "parent");
    assert_eq!(relation.first, "Alice");
    assert_eq!(relation.second, "Bob");
}

#[test]
fn test_parse_multiple_relations() {
    let input = r#"parent("A", "B"). mother("B", "C")."#;
    let (remaining, relations) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 2);
    match &relations[0] {
        DatalogItem::Relation(rel) => {
            assert_eq!(rel.name, "parent");
            assert_eq!(rel.first, "A");
            assert_eq!(rel.second, "B");
        }
        _ => panic!("Expected Relation variant"),
    }
    match &relations[1] {
        DatalogItem::Relation(rel) => {
            assert_eq!(rel.name, "mother");
            assert_eq!(rel.first, "B");
            assert_eq!(rel.second, "C");
        }
        _ => panic!("Expected Relation variant"),
    }
}

#[test]
fn test_parse_fact() {
    let input = r#"male(X)."#;
    let (remaining, fact) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    match &fact[0] {
        DatalogItem::Fact(f) => {
            assert_eq!(f.name, "male");
            assert_eq!(f.first, "X");
        }
        _ => panic!("Expected Fact variant"),
    }
}

#[test]
fn test_parse_rule_definition() {
    let input = r#"parent(X, Y), male(X)"#;
    let (remaining, rule_definition) = parse_rule_definition(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(rule_definition.relations.len(), 2);
    match &rule_definition.relations[0] {
        DatalogItem::Relation(rel) => {
            assert_eq!(rel.name, "parent");
            assert_eq!(rel.first, "X");
            assert_eq!(rel.second, "Y");
        }
        _ => panic!("Expected Relation"),
    };
    match &rule_definition.relations[1] {
        DatalogItem::Fact(rel) => {
            assert_eq!(rel.name, "male");
            assert_eq!(rel.first, "X");
        }
        _ => panic!("Expected Fact"),
    };
}

#[test]
fn test_parse_rule() {
    let input = r#"father(X, Y) :- parent(X, Y), male(X)."#;
    let (remaining, items) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");

    let DatalogItem::Rule(el) = &items[0] else {
        panic!("Expected Rule variant");
    };

    assert_eq!(el.name, "father");
    assert_eq!(el.first, "X");
    assert_eq!(el.second, "Y");
    assert_eq!(el.definition.relations.len(), 2);

    let RuleDefinition { relations } = &el.definition;
    assert_eq!(relations.len(), 2);

    // Check first relation
    let DatalogItem::Relation(rel) = &relations[0] else {
        panic!("Expected Relation");
    };
    assert_eq!(rel.name, "parent");
    assert_eq!(rel.first, "X");
    assert_eq!(rel.second, "Y");

    // Check fact
    let DatalogItem::Fact(rel) = &relations[1] else {
        panic!("Expected Fact");
    };
    assert_eq!(rel.name, "male");
    assert_eq!(rel.first, "X");
}
