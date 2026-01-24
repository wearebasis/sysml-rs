# InterconnectionView design notes (SysML v2)

Source references:
- sysmlv2-references/SysML-v2-Pilot-Implementation/sysml.library/Systems Library/StandardViewDefinitions.sysml
- sysmlv2-references/SysML-v2-Pilot-Implementation/sysml.library/Systems Library/Connections.sysml
- sysmlv2-references/SysML-v2-Pilot-Implementation/sysml.library/Systems Library/Ports.sysml
- sysmlv2-references/SysML-v2-Pilot-Implementation/sysml.library/Systems Library/Interfaces.sysml

## Spec intent summary
InterconnectionView presents exposed features as nodes, nested features as nested
nodes, and connections between features as edges. Boundary features (ports and
parameters) appear on node boundaries. Connections can be elaborated by attached
nodes to show decomposition.

## Rendering decisions (locked)
- Ports are rendered as square boundary symbols.
- Flow direction is derived from port direction (out -> in). If both ends are
  inout or missing, fall back to explicit flow direction on the link.
- Connections are styled by role:
  - connection/interface: solid line
  - flow: solid line with flow color and arrow marker
  - binding/delegation: dashed line, neutral color
- Connection elaboration is rendered as a dashed attachment from the connection
  to an elaboration node. Default attach point is mid-edge; optional anchors
  allow attachment near source/target ports.
- Nodes can have textual and graphical compartments. Textual compartments are
  list-style rows; graphical compartments embed nested nodes.

## VisSpec schema extensions (v0.2 proposal)

### Nodes
Add support for hierarchy and compartments.

```
interface VisSpecNode {
  parentId?: string;               // embed node inside parent
  stereotype?: string;             // rendered in header, e.g. "<<part>>"
  compartments?: VisSpecCompartment[];
}

interface VisSpecCompartment {
  id: string;
  title?: string;                  // e.g. "parts", "ports"
  kind: "text" | "graphical";
  lines?: string[];                // textual list items
  nodeIds?: string[];              // nested nodes in this compartment
  style?: VisSpecStyle;
}
```

### Ports
Add boundary placement, symbol variants, and type annotations.

```
interface VisSpecPort {
  side?: "left" | "right" | "top" | "bottom";
  offset?: number;                 // 0..1 along the side
  symbol?: "square" | "circle" | "lollipop";
  typedBy?: string;                // element id of port type
  multiplicity?: string;           // "1", "0..*", etc
}
```

### Links
Add connection roles, line styles, and elaboration.

```
interface VisSpecLink {
  connectionRole?: "connection" | "interface" | "flow" | "binding" | "delegation";
  lineStyle?: "solid" | "dashed";
  markerStart?: "none" | "triangle" | "diamond" | "open";
  markerEnd?: "none" | "triangle" | "diamond" | "open";

  flowItem?: {
    itemTypeId?: string;
    itemLabel?: string;
    direction?: "sourceToTarget" | "targetToSource" | "bidirectional";
    multiplicity?: string;
  };

  elaboration?: {
    nodeId: string;
    attach?: "edge" | "source" | "target"; // default "edge"
    position?: number;                      // 0..1 along edge if attach=edge
    label?: string;
  };
}
```

## JointJS implementation notes
- Use element embedding for parentId hierarchy.
- Implement port placement using port groups per side with manual offsets.
- Render compartments as internal stacked rectangles; textual compartments are
  text blocks, graphical compartments are embedded child nodes.
- For elaboration, render a dashed link from the connection midpoint to the
  elaboration node. If attach is source/target, anchor near that port.
