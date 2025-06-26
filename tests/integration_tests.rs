use dataloglite::parse_datalog;
use std::fs;

#[test]
fn test_example_datalog() {
    let input = fs::read_to_string("test_examples/example.datalog").expect("Failed to read test file");
    let (remaining, relations) = parse_datalog(&input).expect("Failed to parse");
    
    // After parsing all relations, we should have nothing left (not even whitespace)
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 10);
    
    // Test some sample relations
    assert!(relations.iter().any(|r| r.first == "Alice" && r.second == "Bob"));
    assert!(relations.iter().any(|r| r.first == "Bob" && r.second == "Charlie"));
    assert!(relations.iter().any(|r| r.first == "Diana" && r.second == "Henry"));
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = parse_datalog(input);
    assert!(result.is_err());
}

#[test]
fn test_cousins_facts_rules() {
    let input = fs::read_to_string("test_examples/cousins_facts_rules.datalog")
        .expect("Failed to read test file");
    let (remaining, relations) = parse_datalog(&input).expect("Failed to parse");
    
    assert_eq!(remaining, "");
    assert_eq!(relations.len(), 10);
    
    // Verify some parent relations
    assert!(relations.iter().any(|r| r.first == "Alice" && r.second == "Bob"));
    assert!(relations.iter().any(|r| r.first == "Alice" && r.second == "Barbara"));
    assert!(relations.iter().any(|r| r.first == "Diana" && r.second == "Henry"));
    
    // Verify we didn't parse rules or comments
    assert!(!relations.iter().any(|r| r.first == "father(X, Y)"));
    assert!(!relations.iter().any(|r| r.first == "cousin(X, Y)"));
}
