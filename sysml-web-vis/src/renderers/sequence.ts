import { dia } from "@joint/core";
import { estimateNodeSize, LayoutConfig, layoutSpec } from "../layout";
import {
  SysmlActivationBar,
  SysmlEventOccurrence,
  SysmlLifeline,
  SysmlLink,
  SysmlSequenceFragment,
} from "../shapes";
import { VisSpec, VisSpecNode } from "../vis-spec";
import {
  applyContainerMetadata,
  applyLinkZOrder,
  autoPosition,
  buildNode,
  collectContainerIds,
  layoutGraphicalCompartments,
  makeLinkLabel,
  resolveNodeForLayout,
} from "./common";
import { buildLinkLabels, formatFlowItemSuffix } from "./labels";
import { buildMarker } from "./markers";
import {
  formatKindLabel,
  formatNodeLabel,
  LabelingOptions,
  resolveLabelingOptions,
} from "../notation";
import { CompartmentMode, resolveCompartmentMode } from "../compartments";

export function renderSequenceView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const layout = layoutSpec(spec, layoutConfig);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
  const sequenceLayout = buildSequenceLayout(
    spec,
    layout,
    labelOptions,
    compartmentMode,
  );
  const containerIds = collectContainerIds(spec, {
    kindHints: ["sequence", "lifeline"],
  });
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

  const lifelines = spec.nodes.filter((node) => isLifeline(node));
  const events = spec.nodes.filter((node) => isEventOccurrence(node));
  const containers = spec.nodes.filter((node) => isSequenceContainer(node));
  const fragments = spec.nodes.filter((node) => isFragment(node));
  const activations = spec.nodes.filter((node) => isActivation(node));
  const others = spec.nodes.filter(
    (node) =>
      !isLifeline(node) &&
      !isEventOccurrence(node) &&
      !isSequenceContainer(node) &&
      !isFragment(node) &&
      !isActivation(node),
  );

  containers.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildSequenceContainer(resolvedNode, index, nodeLayout);
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  fragments.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildFragment(resolvedNode, index, nodeLayout);
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  lifelines.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildLifeline(
      resolvedNode,
      index,
      nodeLayout,
      sequenceLayout.lifelineHeight,
    );
    applyContainerMetadata(element, containerIds.has(node.id));
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  activations.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildActivation(resolvedNode, index, nodeLayout, nodes);
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  others.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildNode(resolvedNode, index, nodeLayout);
    element.attr("body/fill", node.style?.fill ?? "var(--sysml-sequence-fill)");
    element.attr("label/text", formatNodeLabel(node, labelOptions));
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  layoutGraphicalCompartments(
    spec,
    nodes,
    nodeLayout.defaultSize,
    compartmentMode,
    {
      preservePositions: layoutContext.allowSpecPositions,
      allowSpecSizes: layoutContext.allowSpecSizes,
    },
  );
  embedChildren(spec, nodes, isEventOccurrence);

  events.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildEvent(resolvedNode, index, nodeLayout, nodes);
    applyContainerMetadata(element, containerIds.has(node.id));
    graph.addCell(element);
    nodes.set(node.id, element);

    embedChild(node, element, nodes);
  });

  spec.links.forEach((linkSpec, index) => {
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
    applySequenceLinkStyle(link, linkSpec);

    const labelOffsetY = index % 2 === 0 ? -16 : -28;
    const explicitLabels = buildLinkLabels(linkSpec.labels, {
      offsetY: labelOffsetY,
    });
    if (explicitLabels.length > 0) {
      link.labels(explicitLabels);
    } else {
      link.labels([makeLinkLabel(linkSpec.kind, 0, 0.5, 0, labelOffsetY)]);
    }

    appendFlowItemLabel(link, linkSpec.flowItem);

    graph.addCell(link);
  });
}

function isLifeline(node: VisSpecNode): boolean {
  return node.kind.toLowerCase().includes("lifeline");
}

function isEventOccurrence(node: VisSpecNode): boolean {
  const kind = node.kind.toLowerCase();
  return (
    kind.includes("event") ||
    kind.includes("occurrence") ||
    kind.includes("messagepoint")
  );
}

function isSequenceContainer(node: VisSpecNode): boolean {
  const kind = node.kind.toLowerCase();
  return kind.includes("sequence") || kind.includes("interaction");
}

function isFragment(node: VisSpecNode): boolean {
  const kind = node.kind.toLowerCase();
  return kind.includes("fragment") || kind.includes("combined");
}

function isActivation(node: VisSpecNode): boolean {
  const kind = node.kind.toLowerCase();
  return kind.includes("activation") || kind.includes("execution");
}

function buildLifeline(
  node: VisSpecNode,
  index: number,
  layout: {
    defaultSize: { width: number; height: number };
    padding: { x: number; y: number };
    gap: { x: number; y: number };
    perRow: number;
    labelOptions?: LabelingOptions;
    compartmentMode?: CompartmentMode;
  },
  lifelineHeight?: number | null,
): dia.Element {
  const element = new SysmlLifeline();
  element.set("id", node.id);
  element.set("sysmlRole", "lifeline");

  const size = node.size ?? {
    width: layout.defaultSize.width,
    height:
      lifelineHeight ??
      estimateNodeSize(
        node,
        layout.defaultSize,
        layout.labelOptions,
        layout.compartmentMode,
      ).height,
  };
  element.resize(size.width, size.height);

  const position = node.position ?? autoPosition(index, layout);
  element.position(position.x, position.y);

  element.attr("kind/text", formatKindLabel(node, layout.labelOptions));
  element.attr("label/text", formatNodeLabel(node, layout.labelOptions));

  const headerHeight = 46;
  element.attr("header/height", headerHeight);
  element.attr("line/x1", size.width / 2);
  element.attr("line/x2", size.width / 2);
  element.attr("line/y1", headerHeight);
  element.attr("line/y2", size.height);

  if (node.style?.fill) {
    element.attr("header/fill", node.style.fill);
  }
  if (node.style?.stroke) {
    element.attr("header/stroke", node.style.stroke);
    element.attr("line/stroke", node.style.stroke);
  }

  return element;
}

function buildSequenceLayout(
  spec: VisSpec,
  layout: ReturnType<typeof layoutSpec>,
  labelOptions: LabelingOptions,
  compartmentMode: CompartmentMode,
): { positions: Map<string, VisSpecPoint>; lifelineHeight: number | null } {
  const positions = new Map(layout.positions);
  if (layout.context.allowSpecPositions) {
    return { positions, lifelineHeight: null };
  }

  const lifelines = spec.nodes.filter((node) => isLifeline(node));
  const events = spec.nodes.filter((node) => isEventOccurrence(node));

  if (lifelines.length > 0) {
    const widths = lifelines.map((node) => {
      const specWidth = layout.context.allowSpecSizes
        ? node.size?.width
        : undefined;
      return (
        specWidth ??
        estimateNodeSize(
          node,
          layout.config.defaultSize,
          labelOptions,
          compartmentMode,
        ).width
      );
    });
    const maxWidth = Math.max(...widths, layout.config.defaultSize.width);
    const baseX = layout.config.padding.x;
    const baseY = layout.config.padding.y;
    const spacing = layout.config.layerGap;

    lifelines.forEach((node, index) => {
      positions.set(node.id, {
        x: baseX + index * (maxWidth + spacing),
        y: baseY,
      });
    });
  }

  let lifelineHeight: number | null = null;
  if (events.length > 0 && lifelines.length > 0) {
    const firstLifeline = lifelines[0];
    const lifelinePos = positions.get(firstLifeline.id);
    const headerHeight = 46;
    const rowGap = Math.max(36, Math.round(layout.config.nodeGap * 0.6));
    const baseY =
      (lifelinePos?.y ?? layout.config.padding.y) + headerHeight + 20;

    events.forEach((node, index) => {
      positions.set(node.id, { x: 0, y: baseY + index * rowGap });
    });

    lifelineHeight = Math.max(
      layout.config.defaultSize.height,
      headerHeight + 20 + events.length * rowGap + 30,
    );
  }

  return { positions, lifelineHeight };
}

function buildSequenceContainer(
  node: VisSpecNode,
  index: number,
  layout: {
    defaultSize: { width: number; height: number };
    padding: { x: number; y: number };
    gap: { x: number; y: number };
    perRow: number;
    labelOptions?: LabelingOptions;
    compartmentMode?: CompartmentMode;
  },
): dia.Element {
  const element = buildNode(node, index, layout);
  element.attr("body/fill", node.style?.fill ?? "var(--sysml-sequence-fill)");
  element.attr("body/stroke", node.style?.stroke ?? "var(--sysml-sequence)");
  element.attr("body/strokeDasharray", "6 4");
  return element;
}

function buildFragment(
  node: VisSpecNode,
  index: number,
  layout: {
    defaultSize: { width: number; height: number };
    padding: { x: number; y: number };
    gap: { x: number; y: number };
    perRow: number;
  },
): dia.Element {
  const element = new SysmlSequenceFragment();
  element.set("id", node.id);
  element.set("sysmlRole", "fragment");

  const size = node.size ?? element.size();
  element.resize(size.width, size.height);

  const position = node.position ?? autoPosition(index, layout);
  element.position(position.x, position.y);

  element.attr("operator/text", node.name ?? node.label ?? "");
  if (node.style?.fill) {
    element.attr("body/fill", node.style.fill);
  }
  if (node.style?.stroke) {
    element.attr("body/stroke", node.style.stroke);
    element.attr("operator/fill", node.style.stroke);
  }

  element.set("z", 0);
  return element;
}

function buildActivation(
  node: VisSpecNode,
  index: number,
  layout: {
    defaultSize: { width: number; height: number };
    padding: { x: number; y: number };
    gap: { x: number; y: number };
    perRow: number;
  },
  nodes: Map<string, dia.Element>,
): dia.Element {
  const element = new SysmlActivationBar();
  element.set("id", node.id);
  element.set("sysmlRole", "activation");

  const size = node.size ?? element.size();
  element.resize(size.width, size.height);

  let position = node.position ?? autoPosition(index, layout);
  if (node.parentId) {
    const parent = nodes.get(node.parentId);
    if (parent) {
      const parentBox = parent.getBBox();
      position = {
        x: parentBox.x + parentBox.width / 2 - size.width / 2,
        y: position.y,
      };
    }
  }

  element.position(position.x, position.y);

  if (node.style?.fill) {
    element.attr("body/fill", node.style.fill);
  }
  if (node.style?.stroke) {
    element.attr("body/stroke", node.style.stroke);
  }

  element.set("z", 5);
  return element;
}

function buildEvent(
  node: VisSpecNode,
  index: number,
  layout: {
    defaultSize: { width: number; height: number };
    padding: { x: number; y: number };
    gap: { x: number; y: number };
    perRow: number;
    labelOptions?: LabelingOptions;
    compartmentMode?: CompartmentMode;
  },
  nodes: Map<string, dia.Element>,
): dia.Element {
  const element = new SysmlEventOccurrence();
  element.set("id", node.id);
  element.set("sysmlRole", "event");

  const size = node.size ?? element.size();
  element.resize(size.width, size.height);

  let position = node.position ?? autoPosition(index, layout);
  if (node.parentId) {
    const parent = nodes.get(node.parentId);
    if (parent) {
      const parentBox = parent.getBBox();
      position = {
        x: parentBox.x + parentBox.width / 2 - size.width / 2,
        y: position.y,
      };
    }
  }

  element.position(position.x, position.y);
  element.attr("label/text", node.name ?? "");

  return element;
}

function applySequenceLinkStyle(link: dia.Link, linkSpec: VisSpecLink): void {
  const normalized = linkSpec.kind.toLowerCase();
  const isSuccession =
    normalized.includes("succession") ||
    normalized.includes("occurrence") ||
    normalized.includes("next");
  const isReturn =
    normalized.includes("return") ||
    normalized.includes("reply") ||
    normalized.includes("response");
  const isAsync =
    normalized.includes("async") ||
    normalized.includes("asynchronous") ||
    normalized.includes("signal");

  const stroke = linkSpec.style?.stroke ?? "var(--sysml-sequence)";
  link.attr("line/stroke", stroke);

  const dashed =
    linkSpec.lineStyle === "dashed"
      ? true
      : linkSpec.lineStyle === "solid"
        ? false
        : isReturn || isSuccession;
  link.attr("line/strokeDasharray", dashed ? "4 4" : null);

  const defaultTargetMarker =
    isReturn || isAsync || isSuccession ? "open" : "triangle";
  link.attr(
    "line/sourceMarker",
    buildMarker(linkSpec.markerStart ?? "none", stroke),
  );
  link.attr(
    "line/targetMarker",
    buildMarker(linkSpec.markerEnd ?? defaultTargetMarker, stroke),
  );
}

function appendFlowItemLabel(
  link: dia.Link,
  flowItem?: {
    itemTypeId?: string;
    itemLabel?: string;
    multiplicity?: string;
  },
): void {
  const flowLabel = formatFlowItemSuffix(flowItem);
  if (!flowLabel) {
    return;
  }

  const labels = link.labels() ?? [];
  labels.push({
    position: 0.7,
    attrs: {
      text: {
        text: flowLabel,
      },
    },
  });
  link.labels(labels);
}

function embedChildren(
  spec: VisSpec,
  nodes: Map<string, dia.Element>,
  skip?: (node: VisSpecNode) => boolean,
): void {
  spec.nodes.forEach((node) => {
    if (!node.parentId) {
      return;
    }

    if (skip && skip(node)) {
      return;
    }

    const child = nodes.get(node.id);
    embedChild(node, child, nodes);
  });
}

function embedChild(
  node: VisSpecNode,
  child: dia.Element | undefined,
  nodes: Map<string, dia.Element>,
): void {
  if (!node.parentId || !child) {
    return;
  }

  const parent = nodes.get(node.parentId);
  if (parent) {
    parent.embed(child);
  }
}
