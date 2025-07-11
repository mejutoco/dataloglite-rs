# Dataloglite

## Run with

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

- Implement queries

  - query facts by quering one parameter
    - `?male('Bob')`

- create a formatter
  - canonical order for
    - facts
    - relations
    - rules
      - right side (rule definitions)
        - maybe each in newline
