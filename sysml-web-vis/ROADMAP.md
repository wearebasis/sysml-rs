# SysMLv2 Diagram Visualization Roadmap (sysml-web-vis)

Scope
- Visualization and rendering in JointJS for sysml-web-vis, including BrowserView
  and GridView.
- Changes focus on visualization and code behavior only (no docs/tests).

Reality check: review vs current code

Gaps
- Definition vs usage styling not enforced; PartDefinition and PartUsage share
  the same rounded block shape.
- Diagram frames are optional and use v1 abbreviations, not view name + subject.
- Port visibility/direction cues are subtle; no default direction/type styling.
- GeneralView lacks category grouping and robust link routing to reduce crossings.
- Grid and Browser views lack legends, multi-relationship cell support, and icons.
- Sequence activation bars and interaction fragments are not implemented.
- Action pin docking is minimal; composite state region styling needs clearer
  separation between header and nested states.

Already implemented / review outdated
- Relationship markers and dashed styles for specialization/composition/satisfy/
  verify/etc are implemented.
- PartUsage label formatting as name: type; TypeOf links are not auto-labeled.
- Interconnection view embeds parts as nodes with ports and flow labels.
- Sequence view has lifelines/events with message/return/async marker styles.
- State view suppresses "StateUsage" kind label by default; supports nested
  substates via graphical compartments.
- External layout strategies (ELK/Dagre/MSAGL) are already wired as optional.

Updated roadmap (visualization/code)

Phase 1: Definition/usage notation normalization
- [x] Add definition/usage styling variants (corner radius, stroke, fill) per
      node kind.
- [x] Centralize label formatting for sizing and rendering so layout matches
      visible labels.
- [x] Optional terminology/case normalization for node.kind and labels.

Phase 2: Diagram framing and metadata
- [x] Frame header uses view name + subject (from viewMetadata) by default,
      with override options.
- [x] Honor compartmentMode for text vs graphical compartments per view.
- [x] Add a legend hook for palette/relationship keys (opt-in).

Phase 3: Port and connection clarity
- [x] Increase port visibility (direction/type styling; lollipop decoration).
- [x] Add explicit TypeOf/InstanceOf link style defaults when present.
- [x] Tune flow item labels and arrows for interconnection views.

Phase 4: View-specific polish
- [x] GeneralView grouping and routing tweaks to reduce crossings; default to
      avoid router when available.
- [x] ActionFlow pin docking and send/accept iconography; improve time/change
      trigger visuals.
- [x] StateTransition composite region layout and divider spacing.
- [x] Sequence activation bars and optional fragments (loop/alt) in fixtures.

Phase 5: Grid/Browser upgrades
- [x] Support multi-relationship cells and stacked labels.
- [x] Gridlines, row grouping, and legend support.
- [x] Browser icons and tree connector styling.

Phase 6: Interconnection layout readability pass
- [ ] Add interconnection-specific layout preset (logical flow columns with
      stable ordering by kind/role: power + sensors left, controller center,
      bus right; constraints/data bottom).
- [ ] Enforce strict orthogonal routing defaults for interconnection links
      (Manhattan + 90-degree turns; consistent step/padding).
- [ ] Port placement rules by direction (in = left/top, out = right/bottom) and
      by flow role to shorten connectors.
- [ ] Label hygiene pass: offset port/link labels off the wire and add minimum
      clearance from node borders.
- [ ] Grid alignment + whitespace tuning for interconnection: consistent row/
      column spacing and avoid dense mid-diagram clustering.

Out of scope (for this roadmap)
- Docs/tests updates and non-visual product UX changes.
- Full editor features beyond basic dragging/selection.
