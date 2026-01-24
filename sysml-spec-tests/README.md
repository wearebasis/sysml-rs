# sysml-spec-tests

## What This Does (One Sentence)

Tests the parser against official SysML examples to make sure it handles real-world files correctly.

## The Problem It Solves

How do we know the parser actually works? We could write test cases ourselves, but that only tests what we *think* SysML looks like. The real test is: **can it parse the official examples?**

The SysML v2 specification includes a corpus of example files — these are the "textbook answers" that define correct SysML. This crate runs all those files through our parser and tracks what works.

Think of it like a **final exam using the official textbook** — we're not grading ourselves; we're using the authority's test cases.

## How It Works

```
    ┌─────────────────────────────────────────────────────────┐
    │           Official SysML v2 Corpus (57 files)           │
    │                                                         │
    │  library.systems/Parts.sysml                            │
    │  library.systems/Actions.sysml                          │
    │  library.systems/Requirements.sysml                     │
    │  ... and 54 more                                        │
    └───────────────────────────┬─────────────────────────────┘
                                │
                                │  Feed each file to
                                ▼
                        ┌───────────────┐
                        │    Parser     │
                        │ (pest-based)  │
                        └───────┬───────┘
                                │
            ┌───────────────────┴───────────────────┐
            ▼                                       ▼
    ┌───────────────┐                       ┌───────────────┐
    │   Success!    │                       │    Failed     │
    │               │                       │               │
    │ File parsed   │                       │ Syntax error  │
    │ correctly     │                       │ at line 42    │
    └───────┬───────┘                       └───────┬───────┘
            │                                       │
            ▼                                       ▼
    ┌─────────────────────────────────────────────────────────┐
    │                   Coverage Report                        │
    │  ═══════════════════════════════════════════════════════ │
    │                                                          │
    │  Passing: 45/57 files (79%)                              │
    │                                                          │
    │  Failing (expected):                                     │
    │    - library.domain/Signals.sysml (signal syntax)        │
    │    - library.domain/Triggers.sysml (trigger syntax)      │
    │    - ... 10 more known gaps                              │
    │                                                          │
    │  Progress: ████████████░░░░ 79%                          │
    └─────────────────────────────────────────────────────────┘
```

## How It Fits Into the System

```
                ┌───────────────────────────┐
                │     Development Cycle     │
                └─────────────┬─────────────┘
                              │
        1. Edit grammar       │
           in pest files      │
                              ▼
                     ┌────────────────┐
                     │ sysml-text-pest│
                     │ (the parser)   │
                     └────────┬───────┘
                              │
        2. Run tests          │
           to check           │
                              ▼
                   ┌──────────────────┐
                   │ sysml-spec-tests │  ← You are here
                   │ (the checker)    │
                   └────────┬─────────┘
                            │
        3. See results       │
                             ▼
              ┌──────────────────────────┐
              │  "52/57 files passing"   │
              │  "Fixed: Triggers.sysml" │
              └──────────────────────────┘
```

## Key Concepts

| Concept | What It Is | Analogy |
|---------|-----------|---------|
| **Corpus** | The official collection of example files | The textbook's practice problems |
| **Coverage** | How many corpus files parse successfully | Your test score |
| **Expected Failures** | Files we know don't work yet (tracked intentionally) | Questions we're still studying |
| **Regression** | When something that worked starts failing | Getting a wrong answer you got right before |

### The Expected Failures File

`data/expected_failures.txt` lists files we know don't parse yet:

```
# These files use signal syntax (not implemented)
library.domain/Signals.sysml
library.domain/Triggers.sysml

# These use advanced allocation (in progress)
library.domain/Allocations.sysml
```

This list should **shrink over time** as we fix parser gaps. If a file suddenly fails that's not on this list, that's a regression — something broke.

## For Developers

<details>
<summary>Running the Tests (click to expand)</summary>

### Quick Unit Tests (No Corpus Needed)

```bash
cargo test -p sysml-spec-tests
```

### Full Corpus Coverage (Requires Spec Bundle)

```bash
# Point to your local copy of the references
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
  cargo test -p sysml-spec-tests -- --ignored

# Or with more output
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
  cargo test -p sysml-spec-tests corpus_coverage -- --ignored --nocapture
```

### Data Files

| File | Purpose |
|------|---------|
| `data/expected_failures.txt` | Files known to not parse yet |
| `data/constructible_kinds.txt` | Element types that can appear in text |
| `data/operators.txt` | Operators for expression coverage |

### Interpreting Results

```
running 1 test
Corpus coverage: 52/57 files passing

UNEXPECTED FAILURES (regressions!):
  - library.domain/Signals.sysml:42 - unexpected 'signal'

EXPECTED FAILURES (known gaps):
  - library.domain/Triggers.sysml - in expected_failures.txt
  - library.domain/Allocations.sysml - in expected_failures.txt
```

- **Unexpected failures** = bugs to fix
- **Expected failures** = known work items

</details>
