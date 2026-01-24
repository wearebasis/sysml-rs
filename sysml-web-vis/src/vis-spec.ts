export type VisSpecVersion = "0.2";

export type VisSpecView =
  | "GeneralView"
  | "InterconnectionView"
  | "StateTransitionView"
  | "ActionFlowView"
  | "SequenceView"
  | "BrowserView"
  | "GridView"
  | "GeometryView";

export interface VisSpec {
  version: VisSpecVersion;
  view: VisSpecView;
  viewMetadata?: VisSpecViewMetadata;
  geometry?: VisSpecGeometry;
  grid?: VisSpecGrid;
  nodes: VisSpecNode[];
  links: VisSpecLink[];
}

export interface VisSpecNode {
  id: string;
  kind: string;
  icon?: string;
  name?: string;
  label?: string;
  parentId?: string;
  container?: boolean;
  stereotype?: string;
  metaclass?: string;
  metadata?: string;
  collapsed?: boolean;
  geometry?: VisSpecNodeGeometry;
  position?: VisSpecPoint;
  size?: VisSpecSize;
  ports?: VisSpecPort[];
  compartments?: VisSpecCompartment[];
  style?: VisSpecStyle;
}

export interface VisSpecPort {
  id: string;
  name: string;
  group?: string;
  kind?: string;
  direction?: "in" | "out" | "inout";
  side?: "left" | "right" | "top" | "bottom";
  offset?: number;
  symbol?: "square" | "circle" | "lollipop";
  typedBy?: string;
  multiplicity?: string;
}

export interface VisSpecLink {
  id: string;
  kind: string;
  source: VisSpecEndpoint;
  target: VisSpecEndpoint;
  connectionRole?:
    | "connection"
    | "interface"
    | "flow"
    | "binding"
    | "delegation";
  lineStyle?: "solid" | "dashed";
  markerStart?: "none" | "triangle" | "diamond" | "open";
  markerEnd?: "none" | "triangle" | "diamond" | "open";
  flowItem?: VisSpecFlowItem;
  elaboration?: VisSpecElaboration;
  transition?: VisSpecTransition;
  geometry?: VisSpecLinkGeometry;
  labels?: VisSpecLabel[];
  style?: VisSpecStyle;
}

export interface VisSpecEndpoint {
  nodeId: string;
  portId?: string;
}

export interface VisSpecLabel {
  text: string;
  position?: number;
}

export interface VisSpecCompartment {
  id: string;
  title?: string;
  kind: "text" | "graphical";
  lines?: string[];
  nodeIds?: string[];
  style?: VisSpecStyle;
}

export interface VisSpecViewMetadata {
  title?: string;
  subtitle?: string;
  description?: string;
  viewpoint?: string;
  subject?: string;
  filterKinds?: string[];
  sortBy?: "name" | "kind";
  sortOrder?: "asc" | "desc";
  compartmentMode?: "textual" | "graphical" | "mixed";
  pinDocking?: boolean;
  labeling?: VisSpecLabeling;
  legend?: VisSpecLegend;
}

export interface VisSpecLabeling {
  kindCase?: "preserve" | "title";
  nameCase?: "preserve" | "definitionUsage";
}

export interface VisSpecLegend {
  title?: string;
  items: string[];
  position?: "top-left" | "top-right" | "bottom-left" | "bottom-right";
}

export interface VisSpecGrid {
  rows: VisSpecGridAxis[];
  columns: VisSpecGridAxis[];
  cells: VisSpecGridCell[];
}

export interface VisSpecGridAxis {
  id: string;
  label: string;
  elementId?: string;
  group?: string;
}

export interface VisSpecGridCell {
  rowId: string;
  columnId: string;
  value?: string;
  values?: string[];
  linkId?: string;
  linkIds?: string[];
  style?: VisSpecStyle;
}

export interface VisSpecFlowItem {
  itemTypeId?: string;
  itemLabel?: string;
  direction?: "sourceToTarget" | "targetToSource" | "bidirectional";
  multiplicity?: string;
}

export interface VisSpecTransition {
  trigger?: string;
  guard?: string;
  effect?: string;
}

export interface VisSpecElaboration {
  nodeId: string;
  attach?: "edge" | "source" | "target";
  position?: number;
  label?: string;
}

export interface VisSpecGeometry {
  frame?: "2d" | "3d";
  units?: string;
  origin?: VisSpecSpatialPoint;
  scale?: number;
}

export interface VisSpecNodeGeometry {
  position?: VisSpecSpatialPoint;
  size?: VisSpecSpatialSize;
  rotation?: VisSpecRotation;
}

export interface VisSpecLinkGeometry {
  points?: VisSpecSpatialPoint[];
  style?: "solid" | "dashed";
}

export interface VisSpecPoint {
  x: number;
  y: number;
}

export interface VisSpecSize {
  width: number;
  height: number;
}

export interface VisSpecSpatialPoint {
  x: number;
  y: number;
  z?: number;
}

export interface VisSpecSpatialSize {
  width: number;
  height: number;
  depth?: number;
}

export interface VisSpecRotation {
  x?: number;
  y?: number;
  z?: number;
}

export interface VisSpecStyle {
  fill?: string;
  stroke?: string;
  text?: string;
}
