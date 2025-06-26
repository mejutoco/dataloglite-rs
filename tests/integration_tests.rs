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
    assert!(relations.iter().any(|r| r.parent == "Alice" && r.child == "Bob"));
    assert!(relations.iter().any(|r| r.parent == "Bob" && r.child == "Charlie"));
    assert!(relations.iter().any(|r| r.parent == "Diana" && r.child == "Henry"));
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = parse_datalog(input);
    assert!(result.is_err());
}
