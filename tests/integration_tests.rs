use dataloglite::{
    parser::{parse_datalog, DatalogItem, RuleDefinition},
    query_engine::interpret,
};
use indoc::indoc;
use std::fs;

#[test]
fn test_example_datalog() {
    let input =
        fs::read_to_string("test_examples/parser/basic.datalog").expect("Failed to read test file");
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
    let input = fs::read_to_string("test_examples/parser/comments.datalog")
        .expect("Failed to read test file");
    let (remaining, relations) = parse_datalog(&input).expect("Failed to parse");

    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 10);
}

#[test]
fn test_cousins_facts_rules() {
    let input = fs::read_to_string("test_examples/parser/cousins_facts_rules.datalog")
        .expect("Failed to read test file");
    let (remaining, items) = parse_datalog(&input).expect("Failed to parse");

    assert_eq!(remaining, "");
    assert_eq!(items.len(), 12);

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
    // print!("Parsed items: {:#?}", items);
    let DatalogItem::Rule(el) = &items[items.len() - 1] else {
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

#[test]
fn test_query_relation() {
    let input = fs::read_to_string("test_examples/queries/basic_relation.datalog")
        .expect("Failed to read test file");

    let mut buffer = Vec::new();
    interpret(&input, &mut buffer);
    let output = String::from_utf8(buffer).expect("Failed to convert output to string");

    let expected_output = indoc! {"
        parent is Alice of Bob
        Query: parent is Alice of Bob?
        true
        Query: parent is Alice of Charlie?
        false"};
    assert_eq!(output.trim(), expected_output)
}

#[test]
fn test_query_variable_based_relation_second_is_var() {
    let input =
        fs::read_to_string("test_examples/queries/variable_based_query_second_is_var.datalog")
            .expect("Failed to read test file");

    let mut buffer = Vec::new();
    interpret(&input, &mut buffer);
    let output = String::from_utf8(buffer).expect("Failed to convert output to string");

    let expected_output = indoc! {"
        parent is Alice of Bob
        parent is Alice of Charlie
        Query: Of whom is Alice parent?
        Bob, Charlie"};
    assert_eq!(output.trim(), expected_output)
}

#[test]
fn test_query_variable_based_relation_first_is_var() {
    let input =
        fs::read_to_string("test_examples/queries/variable_based_query_first_is_var.datalog")
            .expect("Failed to read test file");

    let mut buffer = Vec::new();
    interpret(&input, &mut buffer);
    let output = String::from_utf8(buffer).expect("Failed to convert output to string");

    let expected_output = indoc! {"
        parent is Alice of Bob
        parent is Charlie of Bob
        Query: Who is parent of Bob?
        Alice, Charlie"};
    assert_eq!(output.trim(), expected_output)
}

#[test]
fn test_query_fact() {
    let input = fs::read_to_string("test_examples/queries/basic_fact.datalog")
        .expect("Failed to read test file");

    let mut buffer = Vec::new();
    interpret(&input, &mut buffer);
    let output = String::from_utf8(buffer).expect("Failed to convert output to string");

    let expected_output = indoc! {"
        female is Alice
        male is Bob
        Query: female is Alice
        true
        Query: male is Alice
        false"};
    assert_eq!(output.trim(), expected_output)
}
