# sysml-spec-tests

Parser coverage and corpus validation for SysML v2.

## Purpose

This crate validates parser coverage for `sysml-text-pest` by:

- Parsing the official SysML v2 corpus files
- Tracking rule and element kind coverage
- Keeping an allow-list of expected failures

## Running Tests

Basic tests (no corpus required):

```bash
cargo test -p sysml-spec-tests
```

Corpus tests (requires local spec bundle):

```bash
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
  cargo test -p sysml-spec-tests -- --ignored
```

## Data Files

- `data/expected_failures.txt`: allow-list for known parser gaps
- `data/constructible_kinds.txt`: text-constructible ElementKind list
- `data/operators.txt`: operator list for expression coverage

## Notes

The corpus is intentionally not stored in this repo. Tests will fail unless
`SYSML_CORPUS_PATH` points to a local `sysmlv2-references` checkout.
