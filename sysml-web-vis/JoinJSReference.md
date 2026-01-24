# JointJS Core (Open‑Source) — Comprehensive Data‑Dense Reference (Docs v4.2)
**Code Reference:** ./jointjsreference/ in this directory

**Audience:** a code agent building diagramming features with **JointJS Core** (`@joint/core`) and OSS add-ons, to prevent re‑implementing existing library capabilities.

**Important context:** JointJS documentation is **shared** across **Core** and **JointJS+**. Some pages explicitly say “JointJS+ provides …”; those are **not Core** and are separated in this reference.

---

## 1) Core mental model (what everything hangs off)

- **Model:** `dia.Graph` holds a collection of **cells** (elements + links), their attributes, embeddings, and serialization.
- **View/Surface:** `dia.Paper` renders a graph into SVG, manages cell views, and emits pointer/interaction events.
- **Cells:** `dia.Cell` base → `dia.Element` (nodes) and `dia.Link` (edges).
- **Cell views:** `dia.CellView` + (`ElementView`, `LinkView`) handle rendering, update lifecycle, and DOM events.

---

## 2) Feature topics index (Docs v4.2) with “Core vs JointJS+” guidance

The Features nav in the docs lists the following topics.
This table maps each topic to the **most relevant Core namespaces** (so an agent knows *where to look first*).

> Legend:
> - **Core** = available in `@joint/core` (or in OSS add-on packages called out below).
> - **JointJS+** = docs explicitly say JointJS+ provides it.
> - **UI plugin / verify** = appears to be implemented via `ui.*`, `storage.*`, or `format.*` namespaces in the API navigation; availability depends on your distribution (often JointJS+).

| Docs Feature Topic | Where to look first (API / package) | Availability notes |
|---|---|---|
| Diagram basics | `dia` | Core |
| Ready-to-use shapes | `shapes` | Core |
| Customizing shapes | `dia`, `shapes`, `util`, `V` | Core |
| Ports | `dia` (ports on elements), connection pipeline | Core |
| Labels | `dia.Link` labels; `linkTools` | Core |
| Shape tools | `elementTools`, `linkTools` | Core |
| Containers & Grouping | `dia` embedding / parenting | Core |
| Element palette | `ui.Stencil` | **JointJS+ likely**; drag & drop docs call out `Stencil` as JointJS+.  |
| Property editor and viewer | typically `ui.*` (Inspector-style) | UI plugin / verify |
| Toolbar | typically `ui.*` | UI plugin / verify |
| Zoom & Scroll | `dia.Paper` transforms; also `ui.PaperScroller` exists | UI plugin / verify (PaperScroller is a plugin in `ui`).  |
| Minimap | `ui.Navigator` | UI plugin / verify (Navigator lives in `ui`).  |
| Halo | `ui.Halo` | **JointJS+** (explicit).  |
| Highlighters | `highlighters` | Core |
| Selection | likely `ui.Selection` | UI plugin / verify (in `ui`).  |
| Selection Region | likely `ui.*` | UI plugin / verify |
| Resize & Rotate | likely `ui.*` | UI plugin / verify |
| Undo & Redo | likely plugin / command manager | UI plugin / verify |
| Copy & Paste | likely plugin | UI plugin / verify |
| Keyboard shortcuts | likely plugin | UI plugin / verify |
| Inline text editing | likely `ui.TextEditor` | UI plugin / verify |
| Export & Import | `format` / `storage` namespaces exist | UI plugin / verify (format/storage appear in API nav).  |
| Automatic layouts | `layout` + OSS add-ons below | **Mixed**: Directed Graph + MSAGL are OSS add-ons; others depend on distribution.  |
| Performance | `dia` (paper freeze/unfreeze, view lifecycle), `env` | Core |
| Validations | `dia` + application rules | Core |
| Algorithms | `alg` | **JointJS+** (explicit).  |
| Snap to objects | `dia` + connection pipeline | Core / verify |
| Drag & Drop | `ui.*` (TreeLayoutView/StackLayoutView/Stencil) | **JointJS+** (explicit).  |
| Vector editor | likely `ui.*` | UI plugin / verify |
| Tooltips | likely `ui.*` | UI plugin / verify |
| Popups & Menus | likely `ui.*` | UI plugin / verify |
| Form controls | likely `ui.*` | UI plugin / verify |
| Themes & Styling | `setTheme()`, `dia` attrs | Core |
| Local storage | `storage` | UI plugin / verify (storage namespace in API nav).  |

---

## 3) The “don’t reinvent link routing” pipeline (Core)

If you’re writing custom link geometry, first decide which stage you’re truly missing:

1) **Anchor** → where the link aims in the element’s reference box (`anchors.*`).
2) **Connection point** → final endpoint on the bbox/boundary (`connectionPoints.*`).
3) **Router** → route/vertices (see `routers.*`).
4) **Connector** → SVG path style (see `connectors.*`).
5) **Tools/Highlighters** → interactive handles/feedback (`linkTools`, `elementTools`, `highlighters`).

### 3.1 Anchors (`anchors`)
**Definition:** “An anchor of a link is a point in the reference element that this link wants to reach as its endpoint.”

Built‑ins listed in the anchors API:
- `bottom`, `bottomLeft`, `bottomRight`
- `center` (**default**)
- `left`, `right`
- `top`, `topLeft`, `topRight`
- `midSide` (smart side selection; supports preference modes)
- `modelCenter`
- `perpendicular` (tries to make orthogonal link; otherwise uses `center`)

Common args (many anchors):
- `useModelGeometry` (use model bbox instead of measuring rendered view for performance)
- `rotate`
- `dx` / `dy` offsets support numbers, percentage strings, and `calc()` expressions.

**Quick pattern**
```js
link.source(sourceEl, {
  anchor: {
    name: 'bottom',
    args: { useModelGeometry: true, rotate: true, dx: 10, dy: '40%' }
  }
});
```
(Example pattern mirrors the anchors docs.)

### 3.2 Connection points (`connectionPoints`)
**Definition:** “A link connection point is an endpoint of the link route … usually different from the link anchor point, as it takes into account the presence of the end element.”

Built‑ins called out on the page:
- `anchor` (coincide with anchor, with offset/alignment options)
- `bbox` (**default**; intersection with element bbox)
- `boundary` (intersection with the actual rendered shape boundary; fall back to bbox in some cases)

**Quick pattern**
```js
link.target(targetEl, {
  anchor: { name: 'midSide', args: { padding: 20, mode: 'auto' } },
  connectionPoint: { name: 'boundary', args: { sticky: true } }
});
```
(Uses the concepts and built-in names from the docs; tune args per API page.)

---

## 4) Editing primitives in Core: tools & highlighters

These are the “build blocks” you combine to create rich interactions without writing your own DOM overlay system:

- `elementTools` — tools attached to element views (handles/buttons).
- `linkTools` — tools attached to link views (vertex editing, arrowhead controls, etc).
- `highlighters` — programmatic visual emphasis (selection/hover/validation).

---

## 5) Automatic layouts you can use without JointJS+ (OSS add‑ons)

### 5.1 Directed Graph layout (Dagre) — `@joint/layout-directed-graph` (OSS)
- Provides automatic layout of directed graphs.
- Uses Dagre internally (MIT).
- Exposes a `DirectedGraph` object that includes `layout()` for rearranging the diagram.

### 5.2 MSAGL layout — `@joint/layout-msagl` (OSS)
- Wraps MSAGL’s Sugiyama engine.
- Supports hierarchical routing, bundled splines, nested subgraphs.
- Ships as `@joint/layout-msagl` (JointJS 4.0+); exports a single `layout()` function plus enums.

---

## 6) JointJS+ features explicitly described as JointJS+ (excluded)

These are **not open-source Core**, per docs:

- **`alg` namespace:** “JointJS+ provides plugins for various algorithms such as Dijkstra and PriorityQueue …”
- **Algorithms feature topic:** “JointJS+ provides various algorithms … including shortest path.”
- **Halo:** “JointJS+ provides a Halo plugin …”
- **Drag & Drop:** “JointJS+ provides drag & drop functionality …” including via `TreeLayoutView` / `StackLayoutView` and via the `Stencil` plugin.

---

## 7) Demos (shorthand list) — what to scan before building from scratch

### 7.1 Larger “Open source apps” on the demos index
- UML Use Case Diagram
- Activity Diagram
- DWDM Circuit
- ROI Calculator
- Sequence Diagram
- Isometric Diagram

### 7.2 How to use demos efficiently
- Filter demos to **Open source** first.
- When looking for a feature, search demos by tags corresponding to:
  - Ports / routing / connectors
  - Tools (handles, vertices)
  - Layouts
  - Highlighters

---

## 8) Ultra-short API sidebar map (what modules exist)

The API reference includes (among others): `anchors`, `connectionPoints`, `connectionStrategies`, `connectors`, `dia`, `elementTools`, `g`, `graphUtils`, `highlighters`, `layout`, `linkAnchors`, `linkTools`, `mvc`, `routers`, `setTheme()`, `shapes`, `util`, `V`, `version`, plus `ui`, `storage`, `format`, and `versionPlus`.

> Agent hint: if you’re unsure whether a capability exists, search the API ref for a dedicated namespace first; JointJS tends to put whole subsystems under a named namespace (routers/connectors/anchors/tools/highlighters/layout).

---

*Generated 2026-01-22*
