import { dia } from "@joint/core";
import { LayoutConfig, layoutSpec } from "../layout";
import { SysmlLink } from "../shapes";
import { VisSpec, VisSpecLink, VisSpecNode, VisSpecPort } from "../vis-spec";
import {
  applyLinkZOrder,
  buildNode,
  collectContainerIds,
  layoutGraphicalCompartments,
  makeLinkLabel,
  resolveNodeForLayout,
} from "./common";
import { buildLinkLabels, formatFlowItemLabel } from "./labels";
import { applyParallelLinkOffset, buildParallelLinkIndex } from "./links";
import { buildMarker } from "./markers";
import { resolveLabelingOptions } from "../notation";
import { resolveCompartmentMode } from "../compartments";
import { CONTAINER_PADDING } from "../config/constants";
import { fitElementToChildren } from "../auto-fit";

const INTERCONNECTION_COLUMN_ORDER = ["left", "center", "right"] as const;
type InterconnectionColumn = (typeof INTERCONNECTION_COLUMN_ORDER)[number];
type InterconnectionRole =
  | "source"
  | "process"
  | "distribution"
  | "aux"
  | "neutral";
type PortSide = "left" | "right" | "top" | "bottom";

const INTERCONNECTION_PORT_LABEL_OFFSET = 18;
const INTERCONNECTION_ROUTER_STEP = 18;
const INTERCONNECTION_ROUTER_PADDING = 32;
const INTERCONNECTION_LABEL_OFFSET = -16;

export function renderInterconnectionView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const portLookup = new Map<string, Map<string, VisSpecPort>>();
  const layout = layoutSpec(spec, layoutConfig);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
  const containerIds = collectContainerIds(spec);
  const parallelIndex = buildParallelLinkIndex(spec.links);
  const layoutContext = layout.context;
  const applyToAll = layoutConfig?.applyToAll ?? false;
  const interconnectionHints = buildInterconnectionHints(spec);
  const renderNodes =
    layoutContext.allowSpecPositions || layoutContext.allowSpecSizes
      ? spec.nodes
      : applyInterconnectionPortHints(spec, interconnectionHints);
  const nodeLayout = {
    defaultSize: layout.config.defaultSize,
    padding: layout.config.padding,
    gap: { x: layout.config.layerGap, y: layout.config.nodeGap },
    perRow: layout.config.perRow,
    containerIds,
    labelOptions,
    compartmentMode,
    portLabelOffset: INTERCONNECTION_PORT_LABEL_OFFSET,
  };

  renderNodes.forEach((node, index) => {
    const position = layout.positions.get(node.id);
    const element = buildNode(
      resolveNodeForLayout(node, position, layoutContext),
      index,
      nodeLayout,
    );
    graph.addCell(element);
    nodes.set(node.id, element);

    if (node.ports && node.ports.length > 0) {
      portLookup.set(
        node.id,
        new Map(node.ports.map((port) => [port.id, port])),
      );
    }
  });

  renderNodes.forEach((node) => {
    if (!node.parentId) {
      return;
    }

    const parent = nodes.get(node.parentId);
    const child = nodes.get(node.id);

    if (parent && child) {
      parent.embed(child);
    }
  });

  const preserveCompartments = layoutContext.allowSpecPositions || applyToAll;
  layoutGraphicalCompartments(
    spec,
    nodes,
    layout.config.defaultSize,
    compartmentMode,
    {
      preservePositions: preserveCompartments,
      sizeToChildren: !preserveCompartments,
      perRow: layout.config.perRow,
      allowSpecSizes: layoutContext.allowSpecSizes,
    },
  );
  const shouldUseInterconnectionLayout =
    !layoutContext.allowSpecPositions &&
    !applyToAll &&
    (layoutContext.requestedStrategy === "auto" ||
      layoutContext.requestedStrategy === "interconnection");

  if (shouldUseInterconnectionLayout) {
    layoutInterconnectionChildren(
      spec,
      nodes,
      layout.config,
      interconnectionHints,
    );
    layoutInterconnectionRoots(
      spec,
      nodes,
      layout.config,
      interconnectionHints,
    );
  }

  spec.links.forEach((linkSpec) => {
    const source = nodes.get(linkSpec.source.nodeId);
    const target = nodes.get(linkSpec.target.nodeId);

    if (!source || !target) {
      return;
    }

    const link = new SysmlLink();
    link.set("id", linkSpec.id);
    link.source({ id: source.id, port: linkSpec.source.portId });
    link.target({ id: target.id, port: linkSpec.target.portId });
    applyLinkZOrder(link);
    link.router({
      name: "manhattan",
      args: buildInterconnectionRouterArgs(linkSpec, portLookup),
    });
    link.connector("rounded", { radius: 8 });

    applyLinkStyle(link, linkSpec, portLookup);
    applyParallelLinkOffset(
      link,
      source,
      target,
      parallelIndex.get(linkSpec.id),
    );
    applyLinkLabels(link, linkSpec);

    graph.addCell(link);
    const elaborationLink = renderElaboration(graph, linkSpec, link, nodes);
    if (elaborationLink) {
      applyLinkZOrder(elaborationLink);
    }
  });
}

type InterconnectionHints = {
  columnById: Map<string, InterconnectionColumn>;
  auxIds: Set<string>;
};

function buildInterconnectionHints(spec: VisSpec): InterconnectionHints {
  const columnById = new Map<string, InterconnectionColumn>();
  const auxIds = new Set<string>();

  spec.nodes.forEach((node) => {
    const role = resolveInterconnectionRole(node);
    if (role === "aux") {
      auxIds.add(node.id);
    }
    columnById.set(node.id, resolveInterconnectionColumn(role));
  });

  return { columnById, auxIds };
}

function applyInterconnectionPortHints(
  spec: VisSpec,
  hints: InterconnectionHints,
): VisSpecNode[] {
  const portVotes = collectPortSideVotes(spec, hints);

  return spec.nodes.map((node) => {
    if (!node.ports || node.ports.length === 0) {
      return node;
    }

    const nodeVotes = portVotes.get(node.id);
    const ports = node.ports.map((port) => {
      if (port.side || hasCustomPortGroup(port)) {
        return port;
      }
      const sideHint = nodeVotes ? pickPortSide(nodeVotes.get(port.id)) : null;
      if (sideHint) {
        return { ...port, side: sideHint };
      }
      return port;
    });

    return { ...node, ports: applyPortOffsets(ports) };
  });
}

function collectPortSideVotes(
  spec: VisSpec,
  hints: InterconnectionHints,
): Map<string, Map<string, Map<PortSide, number>>> {
  const votes = new Map<string, Map<string, Map<PortSide, number>>>();

  spec.links.forEach((link) => {
    if (link.source.portId) {
      const side = resolveLinkPortSide(
        link.source.nodeId,
        link.target.nodeId,
        true,
        hints,
      );
      recordPortSideVote(votes, link.source.nodeId, link.source.portId, side);
    }
    if (link.target.portId) {
      const side = resolveLinkPortSide(
        link.source.nodeId,
        link.target.nodeId,
        false,
        hints,
      );
      recordPortSideVote(votes, link.target.nodeId, link.target.portId, side);
    }
  });

  return votes;
}

function recordPortSideVote(
  votes: Map<string, Map<string, Map<PortSide, number>>>,
  nodeId: string,
  portId: string,
  side: PortSide,
): void {
  const nodeVotes =
    votes.get(nodeId) ?? new Map<string, Map<PortSide, number>>();
  const portVotes = nodeVotes.get(portId) ?? new Map<PortSide, number>();
  portVotes.set(side, (portVotes.get(side) ?? 0) + 1);
  nodeVotes.set(portId, portVotes);
  votes.set(nodeId, nodeVotes);
}

function pickPortSide(votes?: Map<PortSide, number>): PortSide | null {
  if (!votes || votes.size === 0) {
    return null;
  }
  let best: PortSide | null = null;
  let bestCount = -1;
  votes.forEach((count, side) => {
    if (count > bestCount) {
      bestCount = count;
      best = side;
    }
  });
  return best;
}

function resolveLinkPortSide(
  sourceId: string,
  targetId: string,
  isSource: boolean,
  hints: InterconnectionHints,
): PortSide {
  const sourceIsAux = hints.auxIds.has(sourceId);
  const targetIsAux = hints.auxIds.has(targetId);

  if (sourceIsAux || targetIsAux) {
    if (isSource) {
      return sourceIsAux ? "top" : "bottom";
    }
    return targetIsAux ? "top" : "bottom";
  }

  const sourceIndex = resolveColumnIndex(
    hints.columnById.get(sourceId) ?? "center",
  );
  const targetIndex = resolveColumnIndex(
    hints.columnById.get(targetId) ?? "center",
  );

  if (sourceIndex !== targetIndex) {
    if (isSource) {
      return sourceIndex < targetIndex ? "right" : "left";
    }
    return sourceIndex < targetIndex ? "left" : "right";
  }

  return isSource ? "bottom" : "top";
}

function applyPortOffsets(ports: VisSpecPort[]): VisSpecPort[] {
  const portsBySide = new Map<PortSide, VisSpecPort[]>();

  ports.forEach((port) => {
    if (hasCustomPortGroup(port)) {
      return;
    }
    const side = resolvePortSide(port);
    const list = portsBySide.get(side) ?? [];
    list.push(port);
    portsBySide.set(side, list);
  });

  return ports.map((port) => {
    if (port.offset !== undefined || hasCustomPortGroup(port)) {
      return port;
    }
    const side = resolvePortSide(port);
    const list = portsBySide.get(side) ?? [];
    if (list.length === 0) {
      return port;
    }
    const index = list.findIndex((item) => item.id === port.id);
    const offset = (index + 1) / (list.length + 1);
    return { ...port, offset };
  });
}

function hasCustomPortGroup(port: VisSpecPort): boolean {
  if (!port.group) {
    return false;
  }
  return !["left", "right", "top", "bottom", "in", "out", "inout"].includes(
    port.group,
  );
}

function resolvePortSide(port: VisSpecPort): PortSide {
  if (port.side) {
    return port.side;
  }
  if (
    port.group === "left" ||
    port.group === "right" ||
    port.group === "top" ||
    port.group === "bottom"
  ) {
    return port.group;
  }
  if (port.group === "in") {
    return "left";
  }
  if (port.group === "out") {
    return "right";
  }
  if (port.group === "inout") {
    return "bottom";
  }
  if (port.direction === "in") {
    return "left";
  }
  if (port.direction === "out") {
    return "right";
  }
  return "bottom";
}

function resolveInterconnectionRole(node: VisSpecNode): InterconnectionRole {
  const haystack = [
    node.kind,
    node.name ?? "",
    node.label ?? "",
    node.stereotype ?? "",
  ]
    .join(" ")
    .toLowerCase();

  if (
    haystack.includes("constraint") ||
    haystack.includes("parameter") ||
    haystack.includes("param") ||
    haystack.includes("data") ||
    haystack.includes("value") ||
    haystack.includes("limit") ||
    haystack.includes("temp")
  ) {
    return "aux";
  }

  if (
    haystack.includes("power") ||
    haystack.includes("sensor") ||
    haystack.includes("input") ||
    haystack.includes("source") ||
    haystack.includes("supply") ||
    haystack.includes("battery")
  ) {
    return "source";
  }

  if (
    haystack.includes("controller") ||
    haystack.includes("control") ||
    haystack.includes("processor") ||
    haystack.includes("compute") ||
    haystack.includes("ecu")
  ) {
    return "process";
  }

  if (
    haystack.includes("bus") ||
    haystack.includes("output") ||
    haystack.includes("actuator") ||
    haystack.includes("display") ||
    haystack.includes("network") ||
    haystack.includes("comm")
  ) {
    return "distribution";
  }

  return "neutral";
}

function resolveInterconnectionColumn(
  role: InterconnectionRole,
): InterconnectionColumn {
  if (role === "source") {
    return "left";
  }
  if (role === "distribution") {
    return "right";
  }
  return "center";
}

function resolveColumnIndex(column: InterconnectionColumn): number {
  return INTERCONNECTION_COLUMN_ORDER.indexOf(column);
}

function layoutInterconnectionChildren(
  spec: VisSpec,
  nodes: Map<string, dia.Element>,
  layoutConfig: {
    layerGap: number;
    nodeGap: number;
    perRow: number;
  },
  hints: InterconnectionHints,
): void {
  const nodeById = new Map(spec.nodes.map((node) => [node.id, node]));
  const childrenByParent = new Map<string, string[]>();

  spec.nodes.forEach((node) => {
    if (!node.parentId) {
      return;
    }
    const list = childrenByParent.get(node.parentId) ?? [];
    list.push(node.id);
    childrenByParent.set(node.parentId, list);
  });

  const columnGap = Math.max(layoutConfig.layerGap, 120);
  const rowGap = Math.max(layoutConfig.nodeGap, 80);

  const depthById = new Map<string, number>();
  const resolving = new Set<string>();
  const resolveDepth = (id: string): number => {
    const cached = depthById.get(id);
    if (cached !== undefined) {
      return cached;
    }
    if (resolving.has(id)) {
      return 0;
    }
    resolving.add(id);
    const parentId = nodeById.get(id)?.parentId;
    const depth =
      parentId && parentId !== id && nodeById.has(parentId)
        ? resolveDepth(parentId) + 1
        : 0;
    resolving.delete(id);
    depthById.set(id, depth);
    return depth;
  };

  const orderedParents = Array.from(childrenByParent.keys()).sort(
    (left, right) => resolveDepth(right) - resolveDepth(left),
  );

  orderedParents.forEach((parentId) => {
    const childIds = childrenByParent.get(parentId) ?? [];
    const parent = nodes.get(parentId);
    if (!parent || Boolean(parent.get("collapsed"))) {
      return;
    }

    const padding = resolveContainerPadding(parent);
    const origin = parent.position();
    const originX = origin.x + padding.left;
    const originY = origin.y + padding.top;
    const columns = new Map<InterconnectionColumn, string[]>();
    INTERCONNECTION_COLUMN_ORDER.forEach((column) => {
      columns.set(column, []);
    });
    const auxIds: string[] = [];

    childIds.forEach((childId) => {
      const column =
        hints.columnById.get(childId) ??
        resolveInterconnectionColumn("neutral");
      if (hints.auxIds.has(childId)) {
        auxIds.push(childId);
        return;
      }
      columns.get(column)?.push(childId);
    });

    const sorter = (left: string, right: string): number => {
      const leftNode = nodeById.get(left);
      const rightNode = nodeById.get(right);
      return compareText(resolveNodeName(leftNode), resolveNodeName(rightNode));
    };

    const activeColumns = INTERCONNECTION_COLUMN_ORDER.filter(
      (column) => (columns.get(column) ?? []).length > 0,
    );
    activeColumns.forEach((column) => {
      columns.get(column)?.sort(sorter);
    });
    auxIds.sort(sorter);

    const columnWidths = new Map<InterconnectionColumn, number>();
    activeColumns.forEach((column) => {
      const width = Math.max(
        ...(columns.get(column) ?? []).map(
          (id) => nodes.get(id)?.size().width ?? 0,
        ),
        0,
      );
      columnWidths.set(column, width);
    });

    const columnOffsets = new Map<InterconnectionColumn, number>();
    let cursorX = originX;
    activeColumns.forEach((column) => {
      columnOffsets.set(column, cursorX);
      cursorX += (columnWidths.get(column) ?? 0) + columnGap;
    });

    let maxColumnHeight = 0;
    activeColumns.forEach((column) => {
      const columnIds = columns.get(column) ?? [];
      let cursorY = originY;
      const columnWidth = columnWidths.get(column) ?? 0;

      columnIds.forEach((id) => {
        const element = nodes.get(id);
        if (!element) {
          return;
        }
        const size = element.size();
        const x =
          (columnOffsets.get(column) ?? originX) +
          Math.max(0, (columnWidth - size.width) / 2);
        element.position(x, cursorY, { sysmlAutoFit: true });
        cursorY += size.height + rowGap;
      });

      if (columnIds.length > 0) {
        maxColumnHeight = Math.max(maxColumnHeight, cursorY - originY - rowGap);
      }
    });

    if (auxIds.length > 0) {
      const perRow = Math.max(1, Math.min(auxIds.length, layoutConfig.perRow));
      const auxStartY = originY + maxColumnHeight + rowGap * 1.5;
      layoutAuxRow(
        auxIds,
        nodes,
        originX,
        auxStartY,
        columnGap,
        rowGap,
        perRow,
      );
    }

    const minRect = parent.get("sysmlMinRect") as
      | { width: number; height: number }
      | undefined;
    fitElementToChildren(parent, {
      padding,
      minRect,
    });
  });
}

function layoutInterconnectionRoots(
  spec: VisSpec,
  nodes: Map<string, dia.Element>,
  layoutConfig: {
    padding: { x: number; y: number };
    layerGap: number;
    nodeGap: number;
    perRow: number;
  },
  hints: InterconnectionHints,
): void {
  const rootIds = spec.nodes
    .filter((node) => !node.parentId)
    .map((node) => node.id)
    .filter((id) => nodes.has(id));
  if (rootIds.length === 0) {
    return;
  }

  const nodeById = new Map(spec.nodes.map((node) => [node.id, node]));
  const columns = new Map<InterconnectionColumn, string[]>();
  INTERCONNECTION_COLUMN_ORDER.forEach((column) => {
    columns.set(column, []);
  });
  const auxIds: string[] = [];

  rootIds.forEach((id) => {
    const column =
      hints.columnById.get(id) ?? resolveInterconnectionColumn("neutral");
    if (hints.auxIds.has(id)) {
      auxIds.push(id);
      return;
    }
    columns.get(column)?.push(id);
  });

  const sorter = (left: string, right: string): number => {
    const leftNode = nodeById.get(left);
    const rightNode = nodeById.get(right);
    return compareText(resolveNodeName(leftNode), resolveNodeName(rightNode));
  };

  const activeColumns = INTERCONNECTION_COLUMN_ORDER.filter(
    (column) => (columns.get(column) ?? []).length > 0,
  );
  activeColumns.forEach((column) => {
    columns.get(column)?.sort(sorter);
  });
  auxIds.sort(sorter);

  const columnGap = Math.max(layoutConfig.layerGap, 140);
  const rowGap = Math.max(layoutConfig.nodeGap, 90);
  const originX = layoutConfig.padding.x;
  const originY = layoutConfig.padding.y;

  const columnWidths = new Map<InterconnectionColumn, number>();
  activeColumns.forEach((column) => {
    const width = Math.max(
      ...(columns.get(column) ?? []).map(
        (id) => nodes.get(id)?.size().width ?? 0,
      ),
      0,
    );
    columnWidths.set(column, width);
  });

  const columnOffsets = new Map<InterconnectionColumn, number>();
  let cursorX = originX;
  activeColumns.forEach((column) => {
    columnOffsets.set(column, cursorX);
    cursorX += (columnWidths.get(column) ?? 0) + columnGap;
  });

  let maxColumnHeight = 0;
  activeColumns.forEach((column) => {
    const columnIds = columns.get(column) ?? [];
    let cursorY = originY;
    const columnWidth = columnWidths.get(column) ?? 0;

    columnIds.forEach((id) => {
      const element = nodes.get(id);
      if (!element) {
        return;
      }
      const size = element.size();
      const x =
        (columnOffsets.get(column) ?? originX) +
        Math.max(0, (columnWidth - size.width) / 2);
      element.position(x, cursorY, { sysmlAutoFit: true });
      cursorY += size.height + rowGap;
    });

    if (columnIds.length > 0) {
      maxColumnHeight = Math.max(maxColumnHeight, cursorY - originY - rowGap);
    }
  });

  if (auxIds.length > 0) {
    const perRow = Math.max(1, Math.min(auxIds.length, layoutConfig.perRow));
    const auxStartY = originY + maxColumnHeight + rowGap * 1.5;
    layoutAuxRow(auxIds, nodes, originX, auxStartY, columnGap, rowGap, perRow);
  }
}

function layoutAuxRow(
  nodeIds: string[],
  nodes: Map<string, dia.Element>,
  originX: number,
  originY: number,
  gapX: number,
  gapY: number,
  perRow: number,
): void {
  let cursorX = originX;
  let cursorY = originY;
  let rowHeight = 0;
  let column = 0;

  nodeIds.forEach((id) => {
    const element = nodes.get(id);
    if (!element) {
      return;
    }
    const size = element.size();
    element.position(cursorX, cursorY, { sysmlAutoFit: true });
    rowHeight = Math.max(rowHeight, size.height);
    column += 1;

    if (column >= perRow) {
      column = 0;
      cursorX = originX;
      cursorY += rowHeight + gapY;
      rowHeight = 0;
    } else {
      cursorX += size.width + gapX;
    }
  });
}

function resolveContainerPadding(element: dia.Element): {
  left: number;
  right: number;
  top: number;
  bottom: number;
} {
  return (
    (element.get("sysmlContainerPadding") as
      | { left: number; right: number; top: number; bottom: number }
      | undefined) ?? CONTAINER_PADDING
  );
}

function resolveNodeName(node?: VisSpecNode): string {
  if (!node) {
    return "";
  }
  return node.name ?? node.label ?? node.id ?? "";
}

function compareText(left: string, right: string): number {
  return left.localeCompare(right, undefined, { sensitivity: "base" });
}

function buildInterconnectionRouterArgs(
  linkSpec: VisSpecLink,
  portLookup: Map<string, Map<string, VisSpecPort>>,
): {
  padding: number;
  step: number;
  maxAllowedDirectionChange: number;
  perpendicular: boolean;
  startDirections: PortSide[];
  endDirections: PortSide[];
} {
  const startSide = resolveEndpointPortSide(linkSpec.source, portLookup);
  const endSide = resolveEndpointPortSide(linkSpec.target, portLookup);
  const allDirections: PortSide[] = ["left", "right", "top", "bottom"];

  return {
    padding: INTERCONNECTION_ROUTER_PADDING,
    step: INTERCONNECTION_ROUTER_STEP,
    maxAllowedDirectionChange: 90,
    perpendicular: true,
    startDirections: startSide ? [startSide] : allDirections,
    endDirections: endSide ? [endSide] : allDirections,
  };
}

function resolveEndpointPortSide(
  endpoint: VisSpecLink["source"],
  portLookup: Map<string, Map<string, VisSpecPort>>,
): PortSide | null {
  if (!endpoint.portId) {
    return null;
  }
  const port = portLookup.get(endpoint.nodeId)?.get(endpoint.portId);
  return port ? resolvePortSide(port) : null;
}

function applyLinkLabels(link: dia.Link, linkSpec: VisSpecLink): void {
  const explicitLabels = buildLinkLabels(linkSpec.labels, {
    offsetY: INTERCONNECTION_LABEL_OFFSET,
    defaultPosition: 0.55,
  });
  if (explicitLabels.length > 0) {
    link.labels(explicitLabels);
    return;
  }

  const flowLabel = formatFlowItemLabel(linkSpec.flowItem);
  if (flowLabel) {
    link.labels([
      makeLinkLabel(flowLabel, 0, 0.55, 0, INTERCONNECTION_LABEL_OFFSET),
    ]);
    return;
  }

  const role = resolveConnectionRole(linkSpec);
  if (role === "flow") {
    return;
  }

  if (shouldAutoLabelInterconnection(linkSpec.kind)) {
    link.labels([
      makeLinkLabel(linkSpec.kind, 0, 0.55, 0, INTERCONNECTION_LABEL_OFFSET),
    ]);
  }
}

function applyLinkStyle(
  link: dia.Link,
  linkSpec: VisSpecLink,
  portLookup: Map<string, Map<string, VisSpecPort>>,
): void {
  const role = resolveConnectionRole(linkSpec);
  const stroke =
    linkSpec.style?.stroke ??
    (role === "flow" ? "var(--sysml-flow)" : "var(--sysml-link-stroke)");

  link.attr("line/stroke", stroke);
  link.attr("line/strokeWidth", role === "flow" ? 2 : 1.4);

  const lineStyle =
    linkSpec.lineStyle ??
    (role === "binding" || role === "delegation" ? "dashed" : "solid");
  link.attr("line/strokeDasharray", lineStyle === "dashed" ? "6 4" : null);

  const direction = resolveFlowDirection(linkSpec, portLookup);
  const defaultMarkers = markerForRole(role, direction, stroke);

  const startMarker = linkSpec.markerStart
    ? buildMarker(linkSpec.markerStart, stroke)
    : defaultMarkers.start;
  const endMarker = linkSpec.markerEnd
    ? buildMarker(linkSpec.markerEnd, stroke)
    : defaultMarkers.end;

  link.attr("line/sourceMarker", startMarker);
  link.attr("line/targetMarker", endMarker);
}

function resolveConnectionRole(
  linkSpec: VisSpecLink,
): VisSpecLink["connectionRole"] {
  if (linkSpec.connectionRole) {
    return linkSpec.connectionRole;
  }

  const kind = linkSpec.kind.toLowerCase();
  if (kind.includes("flow")) {
    return "flow";
  }
  if (kind.includes("bind")) {
    return "binding";
  }
  if (kind.includes("delegate")) {
    return "delegation";
  }
  if (kind.includes("interface")) {
    return "interface";
  }

  return "connection";
}

function resolveFlowDirection(
  linkSpec: VisSpecLink,
  portLookup: Map<string, Map<string, VisSpecPort>>,
): "sourceToTarget" | "targetToSource" | "bidirectional" {
  if (linkSpec.flowItem?.direction) {
    return linkSpec.flowItem.direction;
  }

  const sourceDirection = findPortDirection(linkSpec.source, portLookup);
  const targetDirection = findPortDirection(linkSpec.target, portLookup);

  if (sourceDirection === "out" && targetDirection === "in") {
    return "sourceToTarget";
  }
  if (sourceDirection === "in" && targetDirection === "out") {
    return "targetToSource";
  }
  if (sourceDirection === "inout" || targetDirection === "inout") {
    return "bidirectional";
  }

  return "sourceToTarget";
}

function findPortDirection(
  endpoint: VisSpecLink["source"],
  portLookup: Map<string, Map<string, VisSpecPort>>,
): VisSpecPort["direction"] | undefined {
  if (!endpoint.portId) {
    return undefined;
  }

  return portLookup.get(endpoint.nodeId)?.get(endpoint.portId)?.direction;
}

function markerForRole(
  role: VisSpecLink["connectionRole"],
  direction: "sourceToTarget" | "targetToSource" | "bidirectional",
  stroke: string,
): { start: Record<string, unknown>; end: Record<string, unknown> } {
  if (role !== "flow") {
    return {
      start: buildMarker("none", stroke),
      end: buildMarker("none", stroke),
    };
  }

  const markerStyle = { strokeWidth: 1.6 };

  if (direction === "targetToSource") {
    return {
      start: buildMarker("triangle", stroke, markerStyle),
      end: buildMarker("none", stroke, markerStyle),
    };
  }

  if (direction === "bidirectional") {
    return {
      start: buildMarker("triangle", stroke, markerStyle),
      end: buildMarker("triangle", stroke, markerStyle),
    };
  }

  return {
    start: buildMarker("none", stroke, markerStyle),
    end: buildMarker("triangle", stroke, markerStyle),
  };
}

function shouldAutoLabelInterconnection(kind: string): boolean {
  const normalized = kind.toLowerCase();
  const suppressed = [
    "connection",
    "connector",
    "binding",
    "delegation",
    "interface",
    "flow",
  ];

  return !suppressed.some((term) => normalized.includes(term));
}

function renderElaboration(
  graph: dia.Graph,
  linkSpec: VisSpecLink,
  link: dia.Link,
  nodes: Map<string, dia.Element>,
): dia.Link | null {
  if (!linkSpec.elaboration) {
    return null;
  }

  const target = nodes.get(linkSpec.elaboration.nodeId);
  if (!target) {
    return null;
  }

  const attach = linkSpec.elaboration.attach ?? "edge";
  const position = linkSpec.elaboration.position ?? 0.5;
  const ratio = attach === "source" ? 0 : attach === "target" ? 1 : position;

  const elaborationLink = new SysmlLink();
  elaborationLink.source({
    id: link.id,
    anchor: { name: "connectionRatio", args: { ratio } },
  });
  elaborationLink.target({ id: target.id });
  elaborationLink.router({
    name: "manhattan",
    args: { padding: 12, step: 10 },
  });
  elaborationLink.connector("rounded");
  elaborationLink.attr("line/strokeDasharray", "4 4");
  elaborationLink.attr("line/targetMarker", buildMarker("none"));
  elaborationLink.attr("line/sourceMarker", buildMarker("none"));

  if (linkSpec.elaboration.label) {
    elaborationLink.labels([
      {
        position: 0.5,
        attrs: {
          text: { text: linkSpec.elaboration.label },
        },
      },
    ]);
  }

  graph.addCell(elaborationLink);
  return elaborationLink;
}
