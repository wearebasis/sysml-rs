# sysml-web-vis

Web renderer for SysML v2 views using JointJS Core. This package consumes a
VisSpec JSON payload and renders it in an interactive SVG canvas.

## Quick start

```bash
cd sysml-web-vis
npm install
npm run dev
```

The dev server loads `fixtures/general.json` by default. Use
`?view=InterconnectionView`, `?view=StateTransitionView`,
`?view=ActionFlowView`, `?view=SequenceView`, `?view=BrowserView`,
`?view=GridView`, or `?view=GeometryView` to switch fixtures
without editing code. You can also override layout with
`?layout=grid`, `?layout=layered`, or `?layout=manual`.
Optional layouts (`?layout=dagre|msagl|elk`) and `?router=avoid` require extra
packages; if they are not installed the app will warn and fall back.
Use `?fixture=<id>` to load a specific fixture from the catalog (see
`docs/verification.md`).

Available fixtures:
- `fixtures/general.json`
- `fixtures/general-bdd.json`
- `fixtures/general-requirements.json`
- `fixtures/general-package.json`
- `fixtures/general-usecase.json`
- `fixtures/interconnection.json`
- `fixtures/interconnection-ibd.json`
- `fixtures/interconnection-parametric.json`
- `fixtures/state-transition.json`
- `fixtures/action-flow.json`
- `fixtures/sequence.json`
- `fixtures/browser.json`
- `fixtures/grid.json`
- `fixtures/geometry.json`

## Supported diagram types

- GeneralView: Block Definition, Package, Requirement, Use Case
- InterconnectionView: Internal Block, Parametric
- ActionFlowView: Activity
- StateTransitionView: State Machine
- SequenceView: Sequence
- GeometryView: Geometry/Spatial
- BrowserView/GridView: non-diagram UI (out of scope for notation coverage)

## View mapping

- GeneralView -> Block Definition, Package, Requirement, Use Case diagrams
- InterconnectionView -> Internal Block, Parametric diagrams
- ActionFlowView -> Activity diagrams
- StateTransitionView -> State Machine diagrams
- SequenceView -> Sequence diagrams
- GeometryView -> Geometry/Spatial diagrams
- BrowserView/GridView -> non-diagram compatibility stubs (de-scoped from this module)

## Export

UI export:
Use the export buttons in the control panel to download SVG or PNG snapshots
of the current view. Exports are generated from the current canvas and use the
view metadata title (or view name) for the filename.

CLI export (SVG snapshots + PNGs):

```bash
cd sysml-web-vis
npx playwright install
npm run test:update
npm run test:png
```

SVG snapshots are stored under `tests/snapshots.spec.ts-snapshots` and PNGs
under `tests/png`.

## Snapshot tests

```bash
cd sysml-web-vis
npm install
npx playwright install
npm run test:update
```

This will generate baseline SVG snapshots under
`sysml-web-vis/tests/snapshots.spec.ts-snapshots`. Subsequent runs use
`npm run test` to compare changes.

## VisSpec contract

- Schema: `schema/vis-spec.schema.json`
- Types: `src/vis-spec.ts`
- Fixtures: `fixtures/*.json`

## Design notes

- InterconnectionView: `docs/interconnection-view.md`
- Verification checklist + fixtures: `docs/verification.md`

## JointJS demo references

Open-source demos worth mining for defaults, routing, and interaction patterns:

- Sequence diagrams: https://www.jointjs.com/demos/sequence-diagram
- Activity diagrams (action flow + control nodes): https://www.jointjs.com/demos/activity-diagram
- Use case diagrams (actors + associations): https://www.jointjs.com/demos/use-case-diagram
- Flowchart basics (node/link styling + routing): https://www.jointjs.com/demos/flowchart
- Data mapping (ports + labeled connections): https://www.jointjs.com/demos/data-mapping
- Cables (port magnets + orthogonal routing): https://www.jointjs.com/demos/cables
- Organizational chart (tree layout patterns): https://www.jointjs.com/demos/organizational-chart
- Mind map (hierarchy layout + labels): https://www.jointjs.com/demos/mindmap
- Serpentine layout (edge routing tricks): https://www.jointjs.com/demos/serpentine-layout
- Force-directed layout (auto layout behavior): https://www.jointjs.com/demos/force-directed-layout
- Kitchen sink (misc. UI patterns): https://www.jointjs.com/demos/kitchen-sink-app

## Project layout

```
sysml-web-vis/
  fixtures/        Example VisSpec inputs
  schema/          JSON schema for VisSpec
  src/
    renderers/     View-specific renderers
    shapes/        JointJS base shapes
    app.ts         UI entry point
    bootstrap.ts   Render + layout pipeline
    export.ts      SVG/PNG export helpers
    interaction.ts Drag/drop and constraints
```
