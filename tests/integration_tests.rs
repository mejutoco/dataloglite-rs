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

// #[test]
// fn test_cousins_facts_rules() {
//     let input = fs::read_to_string("test_examples/cousins_facts_rules.datalog")
//         .expect("Failed to read test file");
//     let (remaining, relations) = parse_datalog(&input).expect("Failed to parse");

//     assert_eq!(remaining, "");
//     assert_eq!(relations.len(), 10);

//     // Verify some parent relations
//     assert!(relations.iter().any(|r| match r {
//         DatalogItem::Relation(rel) => rel.first == "Alice" && rel.second == "Bob",
//         _ => false,
//     }));
//     assert!(relations.iter().any(|r| match r {
//         DatalogItem::Relation(rel) => rel.first == "Alice" && rel.second == "Barbara",
//         _ => false,
//     }));
//     assert!(relations.iter().any(|r| match r {
//         DatalogItem::Relation(rel) => rel.first == "Diana" && rel.second == "Henry",
//         _ => false,
//     }));

//     // Verify we didn't parse rules or comments
//     assert!(!relations.iter().any(|r| match r {
//         DatalogItem::Relation(rel) => rel.first == "father(X, Y)",
//         _ => false,
//     }));
//     assert!(!relations.iter().any(|r| match r {
//         DatalogItem::Relation(rel) => rel.first == "cousin(X, Y)",
//         _ => false,
//     }));
// }
