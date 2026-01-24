# sysml-vis Roadmap

This roadmap focuses on **static exports only** (no interactivity). The initial target formats are:

- **DOT (Graphviz)** for structure/trace/overview diagrams
- **PlantUML** for state/action/sequence diagrams

---

## Milestone 0: Baseline Exporters (Now)

**Goal:** Keep the current DOT and PlantUML output stable while we add tests and structure.

Deliverables:
- Snapshot tests for DOT output of a small ModelGraph.
- Snapshot tests for PlantUML output of a small ModelGraph.
- Consistent element labels (kind + name) and relationship labels.

Acceptance:
- `cargo test -p sysml-vis` passes with stable output.

---

## Milestone 1: Graphviz Helpers (Static Rendering)

**Goal:** Provide helper functions that call the Graphviz `dot` binary to render DOT.

Deliverables:
- `render_dot_to_svg(dot: &str) -> Result<String, VisError>`
- `render_dot_to_png(dot: &str, path: &Path) -> Result<(), VisError>`
- `render_dot_to_pdf(dot: &str, path: &Path) -> Result<(), VisError>`
- `GraphvizOptions` (engine: dot/neato/fdp/sfdp; rankdir; node/edge spacing)
- Clear error message when `dot` is not installed

Acceptance:
- Running with Graphviz installed produces SVG/PNG/PDF files.
- If Graphviz is missing, helpers return a clear error (no panic).

Notes:
- This uses `std::process::Command` and does not add a Rust dependency.

---

## Milestone 2: View Templates (DOT)

**Goal:** Support spec-based views with tailored DOT layout and styling.

Views and deliverables:
- **BrowserView** (ownership tree): cluster packages, edges from owner to owned.
- **GeneralView** (generic graph): all elements/relationships with minimal styling.
- **GridView** (trace tables): output as DOT tables for satisfy/verify/trace.
- **InterconnectionView**: parts/ports/connectors with directional styling.

Acceptance:
- Each view has a dedicated `to_dot_*` function.
- Snapshot tests for at least one example per view.

---

## Milestone 3: PlantUML View Support

**Goal:** Provide PlantUML exports for behavior-focused views.

Views and deliverables:
- **StateTransitionView**: states, transitions, guards (PlantUML state diagram).
- **ActionFlowView**: actions, control nodes, flows (PlantUML activity diagram).
- **SequenceView**: lifelines and messages (PlantUML sequence diagram).

Acceptance:
- Each view has a dedicated `to_plantuml_*` function.
- Snapshot tests with minimal examples per view.

---

## Milestone 4: Styling + Layout Config

**Goal:** Centralize styling rules and make them configurable.

Deliverables:
- `StyleTheme` (colors, shapes, fonts, arrow styles).
- `LayoutOptions` shared by DOT + PlantUML.
- Optional presets for "architecture", "requirements", "behavior".

Acceptance:
- One place to adjust styling across exporters.
- Defaults render cleanly without custom config.

---

## Milestone 5: Documentation + Examples

**Goal:** Make it easy to use `sysml-vis` without a parser.

Deliverables:
- Example ModelGraph builders per view.
- Rendered sample outputs committed under `examples/outputs/`.
- Updated crate README with copy-paste examples.

Acceptance:
- Someone can generate diagrams from a programmatically built ModelGraph.
