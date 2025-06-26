use dataloglite::{parse_datalog, parse_relation};
use dataloglite::DatalogItem;

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
fn test_parse_rule() {
    // let input r#"father(X, Y) :- parent(X, Y), male(X)."#;
    let input r#"father(X, Y) :- ."#;
    let (remaining, rule) = parse_datalog(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(rule.name, "father");
    assert_eq!(rule.first, "X");
    assert_eq!(rule.second, "Y");
    assert_eq!(rule.relations.len(), 2);
    assert_eq!(rule.relations[0].name, "parent");
    assert_eq!(rule.relations[0].first, "X");
    assert_eq!(rule.relations[0].second, "Y");
}
