# Dataloglite-rs

This is an implementation (in progress) of a the declarative logic programming language [Datalog](https://en.wikipedia.org/wiki/Datalog). A powerful alternative to SQL.

It includes:

- Parser of facts, relations, rules, and queries.
- Execution of some query types.

## Queries supported

### Basic fact queries

```datalog
male("Charlie").
male("Bob").
female("Alice").
female("Julie").
parent("Alice", "Bob").
parent("Charlie", "Bob").
parent("Alice", "Julie").
parent("Charlie", "Julie").
?parent(X, Y), male(X).
```

### Basic relation queries

```datalog
parent("Alice", "Bob").
?parent("Alice", "Bob").
?parent("Alice", "Charlie").
```

### Conjunctive (and queries)

```datalog
male("Charlie").
male("Bob").
female("Alice").
female("Julie").
parent("Alice", "Bob").
parent("Charlie", "Bob").
parent("Alice", "Julie").
parent("Charlie", "Julie").
?parent(X, Y), male(X).
```

### Projection queries (list all for placeholder) for facts

```datalog
male("Charlie").
male("Bob").
female("Alice").
female("Julie").
parent("Alice", "Bob").
parent("Charlie", "Bob").
parent("Alice", "Julie").
parent("Charlie", "Julie").
?male(_).
```

### Projection queries for relations (both with placeholder in first and second position)

```datalog
parent("Alice", "Bob").
parent("Charlie", "Bob").
?parent(X, "Bob").
```

```datalog
parent("Alice", "Bob").
parent("Alice", "Charlie").
?parent("Alice", X).
```

## How to run

```bash
cargo run test_examples/queries/basic_relation.datalog
```

```bash
cargo run test_examples/cousins_facts_rules.datalog
```

## Run tests with prints

```bash
cargo test -- --nocapture
```

## Custom scripts

```bash
cargo make test_with_coverage
```

Coverage report will be at tarpaulin-report.html

### TODO:

- Add pipeline to run tests
- Implement more query types
- api to query as a rust library
- Fuzz testing
- create a formatter
  - canonical order for
    - facts
    - relations
    - rules
      - right side (rule definitions)
        - maybe each in newline
- create a linter

[![Unit Tests](https://github.com/mejutoco/dataloglite-rs/workflows/Run%20Unit%20Tests/badge.svg)](https://github.com/mejutoco/dataloglite-rs/actions)
