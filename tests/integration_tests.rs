use dataloglite::{parse_datalog, DatalogItem};
use std::fs;

#[test]
fn test_example_datalog() {
    let input =
        fs::read_to_string("test_examples/example.datalog").expect("Failed to read test file");
    let (remaining, relations) = parse_datalog(&input).expect("Failed to parse");

    // After parsing all relations, we should have nothing left (not even whitespace)
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 10);

    // Test some sample relations
    assert!(relations.iter().any(|r| {
        if let DatalogItem::Relation(rel) = r {
            rel.first == "Alice" && rel.second == "Bob"
        } else {
            false
        }
    }));
    assert!(relations.iter().any(|r| match r {
        DatalogItem::Relation(rel) => rel.first == "Bob" && rel.second == "Charlie",
        _ => false,
    }));
    assert!(relations.iter().any(|r| match r {
        DatalogItem::Relation(rel) => rel.first == "Diana" && rel.second == "Henry",
        _ => false,
    }));
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = parse_datalog(input);
    // assert results is empy list
    assert!(result.is_ok());
    assert!(result.unwrap().1.is_empty());
}

#[test]
fn test_example_comments() {
    let input = fs::read_to_string("test_examples/example_comments.datalog")
        .expect("Failed to read test file");
    let (remaining, relations) = parse_datalog(&input).expect("Failed to parse");

    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 10);
}

#[test]
fn test_cousins_facts_rules() {
    let input = fs::read_to_string("test_examples/example_comments.datalog")
        .expect("Failed to read test file");
    let (remaining, items) = parse_datalog(&input).expect("Failed to parse");

    assert_eq!(remaining, "");
    assert_eq!(items.len(), 10);

    // Verify some parent relations
    assert!(items.iter().any(|r| match r {
        DatalogItem::Relation(rel) => rel.first == "Alice" && rel.second == "Bob",
        _ => false,
    }));
    assert!(items.iter().any(|r| match r {
        DatalogItem::Relation(rel) => rel.first == "Alice" && rel.second == "Barbara",
        _ => false,
    }));
    assert!(items.iter().any(|r| match r {
        DatalogItem::Relation(rel) => rel.first == "Diana" && rel.second == "Henry",
        _ => false,
    }));
    print!("Parsed items: {:#?}", items);
    let DatalogItem::Rule(el) = &items[items.len() - 1] else {
        panic!("Expected Rule variant");
    };
    assert_eq!(el.name, "father");
    assert_eq!(el.first, "X");
    assert_eq!(el.second, "Y");
    assert_eq!(el.definition.relations.len(), 2);
}
