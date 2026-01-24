import { dia, g } from "@joint/core";
import { CONTAINER_PADDING } from "../config/constants";
import { LayoutConfig, layoutSpec } from "../layout";
import {
  SysmlBarNode,
  SysmlDecisionNode,
  SysmlFinalNode,
  SysmlLink,
  SysmlStartNode,
} from "../shapes";
import { VisSpec, VisSpecLink, VisSpecNode } from "../vis-spec";
import {
  applyContainerMetadata,
  applyLinkZOrder,
  buildNode,
  collectContainerIds,
  layoutGraphicalCompartments,
  makeLinkLabel,
  resolveNodeForLayout,
  resolveNodePosition,
} from "./common";
import { buildLinkLabels } from "./labels";
import { applyParallelLinkOffset, buildParallelLinkIndex } from "./links";
import { ControlNodeDefinition, createControlNode } from "./positioning";
import { buildMarker } from "./markers";
import { resolveLabelingOptions } from "../notation";
import { resolveCompartmentMode } from "../compartments";

const ACTION_CONTROL_NODES: ControlNodeDefinition[] = [
  {
    matches: (kind) => kind.includes("initial") || kind.includes("start"),
    create: () => new SysmlStartNode(),
  },
  {
    matches: (kind) => kind.includes("final") || kind.includes("terminate"),
    create: () => new SysmlFinalNode(),
  },
  {
    matches: (kind) => kind.includes("fork") || kind.includes("join"),
    create: () => new SysmlBarNode(),
  },
  {
    matches: (kind) => kind.includes("decision") || kind.includes("merge"),
    create: () => new SysmlDecisionNode(),
    afterCreate: (element, node) => {
      const name = node.name ?? node.label ?? "";
      if (name) {
        element.attr("label/text", name);
      }
    },
  },
];

const ACTION_ICON_OFFSET = "translate(12, 12)";
const ACTION_ICONS = {
  send: {
    d: "M 0 2 L 10 6 L 0 10 Z",
    fill: "var(--sysml-action)",
    stroke: "var(--sysml-action)",
  },
  accept: {
    d: "M 10 2 L 0 6 L 10 10 Z",
    fill: "var(--sysml-action)",
    stroke: "var(--sysml-action)",
  },
  time: {
    d: "M 6 0 A 6 6 0 1 1 5.999 0 M 6 2 L 6 6 L 9 6",
    fill: "none",
    stroke: "var(--sysml-action)",
    strokeWidth: 1.4,
  },
  change: {
    d: "M 1 2 L 5 0 L 9 2 L 13 0 L 9 6 L 13 8 L 9 10 L 5 8 L 1 10",
    fill: "none",
    stroke: "var(--sysml-action)",
    strokeWidth: 1.4,
  },
} as const;

export function renderActionFlowView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const layout = layoutSpec(spec, layoutConfig);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
  const containerIds = collectContainerIds(spec, {
    kindHints: ["partition", "swimlane", "lane"],
  });
  const parallelIndex = buildParallelLinkIndex(spec.links);
  const nodeLayout = {
    defaultSize: layout.config.defaultSize,
    padding: layout.config.padding,
    gap: { x: layout.config.layerGap, y: layout.config.nodeGap },
    perRow: layout.config.perRow,
    containerIds,
    labelOptions,
    compartmentMode,
  };
  const layoutContext = layout.context;

  spec.nodes.forEach((node, index) => {
    const position = layout.positions.get(node.id);
    const resolvedPosition = resolveNodePosition(node, position, layoutContext);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element =
      createControlNode(
        resolvedNode,
        ACTION_CONTROL_NODES,
        resolvedPosition,
        layout.config.defaultSize,
      ) ?? buildNode(resolvedNode, index, nodeLayout);
    applyActionStyling(element, node);
    applyContainerMetadata(element, containerIds.has(node.id));
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  spec.nodes.forEach((node) => {
    if (!node.parentId) {
      return;
    }

    const parent = nodes.get(node.parentId);
    const child = nodes.get(node.id);

    if (parent && child) {
      parent.embed(child);
    }
  });

  if (!layoutContext.allowSpecPositions) {
    layoutActionFlowChildren(spec, nodes, layout.config);
  }

  layoutGraphicalCompartments(
    spec,
    nodes,
    layout.config.defaultSize,
    compartmentMode,
    {
      preservePositions: layoutContext.allowSpecPositions,
      sizeToChildren: !layoutContext.allowSpecPositions,
      perRow: layout.config.perRow,
      allowSpecSizes: layoutContext.allowSpecSizes,
    },
  );

  if (spec.viewMetadata?.pinDocking !== false) {
    dockActionPins(spec, nodes, layoutContext.allowSpecPositions);
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
    link.router({ name: "manhattan", args: { padding: 20, step: 16 } });
    link.connector("rounded");
    applyParallelLinkOffset(
      link,
      source,
      target,
      parallelIndex.get(linkSpec.id),
    );
    const stroke =
      linkSpec.style?.stroke ?? resolveActionFlowStroke(linkSpec.kind);
    link.attr("line/stroke", stroke);
    link.attr(
      "line/strokeDasharray",
      linkSpec.lineStyle === "dashed" ? "6 4" : null,
    );
    link.attr(
      "line/sourceMarker",
      buildMarker(linkSpec.markerStart ?? "none", stroke),
    );
    link.attr(
      "line/targetMarker",
      buildMarker(linkSpec.markerEnd ?? "open", stroke),
    );

    const explicitLabels = buildLinkLabels(linkSpec.labels);
    if (explicitLabels.length > 0) {
      link.labels(explicitLabels);
    } else {
      if (shouldAutoLabelActionFlow(linkSpec.kind)) {
        link.labels([makeLinkLabel(linkSpec.kind, 0, 0.5)]);
      }
    }

    graph.addCell(link);
  });
}

function shouldAutoLabelActionFlow(kind: string): boolean {
  const normalized = kind.toLowerCase();
  if (normalized.includes("controlflow")) {
    return false;
  }
  if (normalized.includes("flow")) {
    return false;
  }
  return true;
}

function resolveActionFlowStroke(kind: string): string {
  const normalized = kind.toLowerCase();
  if (normalized.includes("object")) {
    return "var(--sysml-flow)";
  }
  return "var(--sysml-action)";
}

function applyActionStyling(element: dia.Element, node: VisSpecNode): void {
  const kind = node.kind.toLowerCase();
  resetActionIcon(element);

  if (kind.includes("action")) {
    element.attr("body/fill", "var(--sysml-action-accent)");
    element.attr("body/stroke", "var(--sysml-action)");
  }

  if (kind.includes("send")) {
    element.attr("body/fill", "var(--sysml-action-accent)");
    element.attr("body/stroke", "var(--sysml-action)");
    element.attr("body/strokeDasharray", "6 3");
    applyActionIcon(element, ACTION_ICONS.send);
    if (!node.stereotype) {
      element.attr("kind/text", "<<send>>");
    }
  }

  if (kind.includes("accept")) {
    element.attr("body/fill", "var(--sysml-node-fill)");
    element.attr("body/stroke", "var(--sysml-action)");
    element.attr("body/strokeDasharray", "2 3");
    applyActionIcon(element, ACTION_ICONS.accept);
    if (!node.stereotype) {
      element.attr("kind/text", "<<accept>>");
    }
  }

  if (kind.includes("parameter") || kind.includes("pin")) {
    element.attr("body/fill", "var(--sysml-param-fill)");
    element.attr("body/stroke", "var(--sysml-action)");
    element.attr("body/rx", 14);
    element.attr("body/ry", 14);
    element.attr("kind/fontSize", 10);
    element.attr("label/fontSize", 12);

    if (!node.size) {
      element.resize(140, 44);
    }
  }

  if (
    kind.includes("trigger") ||
    kind.includes("time") ||
    kind.includes("change")
  ) {
    element.attr("body/fill", "var(--sysml-trigger-fill)");
    element.attr("body/stroke", "var(--sysml-action)");
    element.attr("body/rx", 14);
    element.attr("body/ry", 14);
    element.attr("kind/fontSize", 10);
    element.attr("label/fontSize", 12);
    if (kind.includes("time")) {
      element.attr("body/strokeDasharray", "4 3");
      applyActionIcon(element, ACTION_ICONS.time);
    } else if (kind.includes("change")) {
      element.attr("body/strokeDasharray", "6 2");
      applyActionIcon(element, ACTION_ICONS.change);
    } else {
      element.attr("body/strokeDasharray", "2 4");
    }
    if (!node.size) {
      element.resize(150, 44);
    }
    if (!node.stereotype) {
      element.attr(
        "kind/text",
        kind.includes("time")
          ? "<<time>>"
          : kind.includes("change")
            ? "<<change>>"
            : "<<trigger>>",
      );
    }
  }

  if (node.style?.fill) {
    element.attr("body/fill", node.style.fill);
  }

  if (node.style?.stroke) {
    element.attr("body/stroke", node.style.stroke);
  }

  if (node.style?.text) {
    element.attr("label/fill", node.style.text);
  }

  if (kind.includes("partition") || kind.includes("swimlane")) {
    element.attr("body/fill", "var(--sysml-lane-fill)");
    element.attr("body/stroke", "var(--sysml-lane-stroke)");
    element.attr("body/strokeDasharray", "6 4");
    element.attr("body/rx", 12);
    element.attr("body/ry", 12);

    element.attr("kind/textAnchor", "start");
    element.attr("kind/refX", 16);
    element.attr("label/textAnchor", "start");
    element.attr("label/refX", 16);
    element.attr("label/refY", 34);
  }
}

type ContainerPadding = {
  left: number;
  right: number;
  top: number;
  bottom: number;
};

function layoutActionFlowChildren(
  spec: VisSpec,
  nodes: Map<string, dia.Element>,
  layout: { perRow: number; layerGap: number; nodeGap: number },
): void {
  const childrenByParent = new Map<string, string[]>();
  const compartmentNodes = collectGraphicalCompartmentNodeIds(spec);
  const perRow = Math.max(1, layout.perRow);
  const gapX = layout.layerGap;
  const gapY = layout.nodeGap;

  spec.nodes.forEach((node) => {
    if (!node.parentId) {
      return;
    }
    const list = childrenByParent.get(node.parentId) ?? [];
    list.push(node.id);
    childrenByParent.set(node.parentId, list);
  });

  childrenByParent.forEach((childIds, parentId) => {
    const parent = nodes.get(parentId);
    if (!parent) {
      return;
    }
    const padding = resolveContainerPadding(parent);
    const origin = parent.position();
    let cursorX = origin.x + padding.left;
    let cursorY = origin.y + padding.top;
    let rowHeight = 0;
    let column = 0;

    childIds.forEach((childId) => {
      if (compartmentNodes.has(childId)) {
        return;
      }
      const child = nodes.get(childId);
      if (!child) {
        return;
      }
      const size = child.size();
      child.position(cursorX, cursorY, { sysmlAutoFit: true });
      rowHeight = Math.max(rowHeight, size.height);
      column += 1;
      if (column >= perRow) {
        column = 0;
        cursorX = origin.x + padding.left;
        cursorY += rowHeight + gapY;
        rowHeight = 0;
      } else {
        cursorX += size.width + gapX;
      }
    });
  });
}

function collectGraphicalCompartmentNodeIds(spec: VisSpec): Set<string> {
  const ids = new Set<string>();
  spec.nodes.forEach((node) => {
    node.compartments?.forEach((compartment) => {
      if (compartment.kind !== "graphical") {
        return;
      }
      compartment.nodeIds?.forEach((id) => ids.add(id));
    });
  });
  return ids;
}

function resolveContainerPadding(element: dia.Element): ContainerPadding {
  return (
    (element.get("sysmlContainerPadding") as ContainerPadding | undefined) ??
    CONTAINER_PADDING
  );
}

type ActionIconSpec = (typeof ACTION_ICONS)[keyof typeof ACTION_ICONS];

function applyActionIcon(element: dia.Element, icon: ActionIconSpec): void {
  element.attr("icon/display", "block");
  element.attr("icon/d", icon.d);
  element.attr("icon/transform", ACTION_ICON_OFFSET);
  element.attr("icon/fill", icon.fill ?? "none");
  element.attr("icon/stroke", icon.stroke ?? "var(--sysml-action)");
  element.attr("icon/strokeWidth", icon.strokeWidth ?? 1.4);
}

function resetActionIcon(element: dia.Element): void {
  element.attr("icon/display", "none");
  element.attr("icon/d", "");
}

function dockActionPins(
  spec: VisSpec,
  nodes: Map<string, dia.Element>,
  allowManualPosition: boolean,
): void {
  const actionIds = new Set(
    spec.nodes.filter((node) => isActionNode(node)).map((node) => node.id),
  );
  const pinNodes = spec.nodes.filter((node) => isPinNode(node));
  const placement = new Map<
    string,
    { left: number; right: number; bottom: number }
  >();

  pinNodes.forEach((pin) => {
    const actionId = findConnectedAction(pin.id, actionIds, spec.links);
    if (!actionId) {
      return;
    }

    const actionElement = nodes.get(actionId);
    const pinElement = nodes.get(pin.id);
    if (!actionElement || !pinElement) {
      return;
    }

    const actionBox = actionElement.getBBox();
    const pinBox = pinElement.getBBox();
    if (
      allowManualPosition &&
      pin.position &&
      !boxesOverlap(actionBox, pinBox, 6)
    ) {
      return;
    }

    const parent = pin.parentId ? nodes.get(pin.parentId) : undefined;
    const parentBox = parent ? parent.getBBox() : undefined;
    const pinSize = pinElement.size();
    const gap = 10;
    const offset = 16;
    const preferredSide = resolvePinDockSide(pin, actionIds, spec.links);
    const side = resolveDockSide(
      preferredSide,
      actionBox,
      parentBox,
      pinSize,
      offset,
    );
    if (!side) {
      return;
    }

    const slot = nextPinSlot(placement, actionId, side);
    const docked = buildDockedPinPosition(
      side,
      actionBox,
      pinSize,
      slot,
      offset,
      gap,
    );
    const next = parentBox
      ? {
          x: clamp(
            docked.x,
            parentBox.x + 8,
            parentBox.x + parentBox.width - pinSize.width - 8,
          ),
          y: clamp(
            docked.y,
            parentBox.y + 60,
            parentBox.y + parentBox.height - pinSize.height - 8,
          ),
        }
      : docked;

    pinElement.position(next.x, next.y);
  });
}

function resolvePinDockSide(
  pin: VisSpecNode,
  actionIds: Set<string>,
  links: VisSpecLink[],
): "left" | "right" | "bottom" {
  const stereotype = pin.stereotype?.toLowerCase();
  if (stereotype) {
    if (stereotype.includes("inout")) {
      return "bottom";
    }
    if (stereotype.includes("in")) {
      return "left";
    }
    if (stereotype.includes("out")) {
      return "right";
    }
  }

  let seenIncoming = false;
  let seenOutgoing = false;
  links.forEach((link) => {
    if (link.source.nodeId === pin.id && actionIds.has(link.target.nodeId)) {
      seenOutgoing = true;
    }
    if (link.target.nodeId === pin.id && actionIds.has(link.source.nodeId)) {
      seenIncoming = true;
    }
  });

  if (seenIncoming && seenOutgoing) {
    return "bottom";
  }
  if (seenOutgoing) {
    return "left";
  }
  if (seenIncoming) {
    return "right";
  }

  return "bottom";
}

function resolveDockSide(
  preferred: "left" | "right" | "bottom",
  actionBox: g.Rect,
  parentBox: g.Rect | undefined,
  pinSize: { width: number; height: number },
  offset: number,
): "left" | "right" | "bottom" | null {
  const order = buildDockSideOrder(preferred);
  for (const side of order) {
    if (canDockPin(side, actionBox, parentBox, pinSize, offset)) {
      return side;
    }
  }
  return null;
}

function buildDockSideOrder(
  preferred: "left" | "right" | "bottom",
): Array<"left" | "right" | "bottom"> {
  if (preferred === "left") {
    return ["left", "right", "bottom"];
  }
  if (preferred === "right") {
    return ["right", "left", "bottom"];
  }
  return ["bottom", "left", "right"];
}

function canDockPin(
  side: "left" | "right" | "bottom",
  actionBox: g.Rect,
  parentBox: g.Rect | undefined,
  pinSize: { width: number; height: number },
  offset: number,
): boolean {
  if (!parentBox) {
    return true;
  }

  const padding = 8;
  if (side === "left") {
    return actionBox.x - offset - pinSize.width >= parentBox.x + padding;
  }
  if (side === "right") {
    return (
      actionBox.x + actionBox.width + offset + pinSize.width <=
      parentBox.x + parentBox.width - padding
    );
  }

  return (
    actionBox.y + actionBox.height + offset + pinSize.height <=
    parentBox.y + parentBox.height - padding
  );
}

function buildDockedPinPosition(
  side: "left" | "right" | "bottom",
  actionBox: g.Rect,
  pinSize: { width: number; height: number },
  slot: number,
  offset: number,
  gap: number,
): { x: number; y: number } {
  let x = actionBox.x;
  let y = actionBox.y + 28 + slot * (pinSize.height + gap);

  if (side === "left") {
    x = actionBox.x - pinSize.width - offset;
  } else if (side === "right") {
    x = actionBox.x + actionBox.width + offset;
  } else {
    x = actionBox.x + (actionBox.width - pinSize.width) / 2;
    y = actionBox.y + actionBox.height + offset + slot * (pinSize.height + gap);
  }

  return { x, y };
}

function findConnectedAction(
  pinId: string,
  actionIds: Set<string>,
  links: VisSpecLink[],
): string | null {
  for (const link of links) {
    if (link.source.nodeId === pinId && actionIds.has(link.target.nodeId)) {
      return link.target.nodeId;
    }
    if (link.target.nodeId === pinId && actionIds.has(link.source.nodeId)) {
      return link.source.nodeId;
    }
  }
  return null;
}

function nextPinSlot(
  placement: Map<string, { left: number; right: number; bottom: number }>,
  actionId: string,
  side: "left" | "right" | "bottom",
): number {
  const current = placement.get(actionId) ?? { left: 0, right: 0, bottom: 0 };
  const slot = current[side];
  current[side] += 1;
  placement.set(actionId, current);
  return slot;
}

function isPinNode(node: VisSpecNode): boolean {
  const kind = node.kind.toLowerCase();
  return kind.includes("parameter") || kind.includes("pin");
}

function isActionNode(node: VisSpecNode): boolean {
  const kind = node.kind.toLowerCase();
  return kind.includes("action");
}

function boxesOverlap(a: g.Rect, b: g.Rect, padding = 0): boolean {
  const overlapX = Math.min(a.x + a.width, b.x + b.width) - Math.max(a.x, b.x);
  const overlapY =
    Math.min(a.y + a.height, b.y + b.height) - Math.max(a.y, b.y);
  return overlapX > padding && overlapY > padding;
}

function clamp(value: number, min: number, max: number): number {
  if (value < min) {
    return min;
  }
  if (value > max) {
    return max;
  }
  return value;
}
