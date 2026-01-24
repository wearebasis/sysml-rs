# Verification and Fixtures

## Fixture inventory

Diagram fixtures (modeled after common SysML reference examples):
- `general` -> `fixtures/general.json` (GeneralView): combined BDD, package, requirement, and use case coverage.
- `general-bdd` -> `fixtures/general-bdd.json` (GeneralView): BDD with composition, aggregation, generalization, association, and block compartments.
- `general-requirements` -> `fixtures/general-requirements.json` (GeneralView): requirement IDs/text with satisfy/verify/derive/refine/allocate links.
- `general-package` -> `fixtures/general-package.json` (GeneralView): packages with contains, import, merge dependencies.
- `general-usecase` -> `fixtures/general-usecase.json` (GeneralView): actors, use cases, include/extend relationships.
- `interconnection` -> `fixtures/interconnection.json` (InterconnectionView): combined IBD and parametric coverage.
- `interconnection-ibd` -> `fixtures/interconnection-ibd.json` (InterconnectionView): context block, parts, ports, item flows.
- `interconnection-parametric` -> `fixtures/interconnection-parametric.json` (InterconnectionView): constraint blocks, parameter nodes, binding connectors.
- `action-flow` -> `fixtures/action-flow.json` (ActionFlowView): activity flow with partitions, control/object flows.
- `state-transition` -> `fixtures/state-transition.json` (StateTransitionView): state machine with pseudostates and transitions.
- `sequence` -> `fixtures/sequence.json` (SequenceView): lifelines with sync/async/return messages.
- `geometry` -> `fixtures/geometry.json` (GeometryView): spatial nodes with explicit geometry links.

Non-diagram fixtures (UI only, out of SysML notation scope):
- `browser` -> `fixtures/browser.json`
- `grid` -> `fixtures/grid.json`

Use `?fixture=<id>` to load a specific fixture in the dev server.

## Arrowhead and relationship checklist

GeneralView (structural + requirements):
- Composition: filled diamond at source (whole), no arrow at target.
- Aggregation: hollow diamond at source (whole), no arrow at target.
- Generalization/Specialization: hollow triangle at target (supertype), source is subtype.
- TypeOf/Typing: open arrow at target (type).
- Contains/Ownership: dashed line, no arrowhead.
- Import/Merge/Access: dashed line, open arrow at target.
- Satisfy/Verify/Derive/Allocate/Refine/Trace/Depend: dashed line, open arrow at target.
- Association: solid line, no arrowhead by default.

Use case relationships:
- Include/Extend: dashed line with open arrow at target use case, label <<include>> or <<extend>>.
- Actor association: solid line, no arrowhead.

Interconnection/Parametric:
- Flow: triangle arrow at target for source-to-target, triangle at source for target-to-source, both for bidirectional.
- Connection/Interface: solid line, no arrowhead.
- Binding/Delegation: dashed line, no arrowhead.

Action flow (Activity):
- Control flow: solid line, open arrow at target.
- Object flow: solid line, open arrow at target with flow styling.

State machine:
- Transition: solid line with triangle at target state.
- Initial/Final: check that transitions connect to correct pseudostates.

Sequence:
- Synchronous message: solid line with filled triangle.
- Asynchronous/signal: solid line with open arrow.
- Return/reply: dashed line with open arrow.
- Succession/occurrence: dashed line with open arrow.

Geometry:
- Geometry links: triangle marker at target unless explicitly overridden; dashed if geometry style is dashed.

## Snapshot and export workflow

CLI export (SVG snapshots + PNGs):
- `npm run test:update` to render all fixtures to `tests/snapshots.spec.ts-snapshots`.
- `npm run test:png` to convert snapshots to `tests/png`.

When adding a fixture:
- Register it in `src/fixtures.ts`.
- Re-run `npm run test:update` and `npm run test:png`.
