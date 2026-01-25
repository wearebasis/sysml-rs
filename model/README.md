# SysmlRs Model (SysML v2 Text)

This directory contains a SysML v2 textual model that represents the **sysml-rs** workspace: crates, high‑level dependencies, and key trait→implementation relationships.

## Files

- `sysml-rs.sysml` — The model (crates + dependencies + trait/impl examples)

## Conventions Used

- **Crates** are modeled as `part def` instances of a base `Crate` definition.
- **Trait-like abstractions** are modeled as `abstract part def`.
- **Implementations** are modeled as `part def` specializing the abstract part (`:>`).
- Each crate specializes two category types:
  - **Status**: `ImplementedCrate | PartialCrate | PlaceholderCrate`
  - **Layer**: `FoundationsCrate | CoreCrate | TextCrate | IdeCrate | FeaturesCrate | StorageCrate | ApiCrate | ToolingCrate | TestingCrate`

## Demo: Parse → Resolve → Diagnostics

This uses the example binary to parse the model and run name resolution.

```bash
cargo run --example parse_sysml_model
```

Expected output:
- Parse diagnostics (ideally 0 errors)
- Resolution summary (resolved/unresolved counts)

## Notes

- Dependencies are intentionally **high-level** and follow Cargo/DEP rules.
- Non-implemented crates are present but minimal (no deep modeling).
- The model is intended for demos and diagrams, not spec-authoritative modeling.
