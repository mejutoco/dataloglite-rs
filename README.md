## Dataloglite

Run with

```bash
cargo run test_examples/query_basic_relation.datalog
```

```bash
cargo run test_examples/cousins_facts_rules.datalog
```

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
