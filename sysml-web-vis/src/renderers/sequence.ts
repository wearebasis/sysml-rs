import { dia } from "@joint/core";
import { estimateNodeSize, LayoutConfig, layoutSpec } from "../layout";
import {
  SysmlActivationBar,
  SysmlEventOccurrence,
  SysmlLifeline,
  SysmlLink,
  SysmlSequenceFragment,
} from "../shapes";
import type {
  VisSpec,
  VisSpecCompartment,
  VisSpecLink,
  VisSpecNode,
  VisSpecPoint,
  VisSpecSize,
} from "../vis-spec";
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
import {
  CompartmentMode,
  filterTextCompartments,
  resolveCompartmentMode,
} from "../compartments";
import { GRAPHICAL_COMPARTMENT } from "../config/constants";

const SEQUENCE_HEADER_HEIGHT = 46;
const EVENT_DIAMETER = 14;
const FRAGMENT_PADDING = 24;
const FRAGMENT_HEADER_HEIGHT = 24;
const MIN_FRAGMENT_HEIGHT = 100;
const MIN_FRAGMENT_WIDTH = 140;
const NESTED_ACTIVATION_PADDING = 8;

type SequenceLayoutResult = {
  positions: Map<string, VisSpecPoint>;
  lifelineHeight: number | null;
  activationSizes: Map<string, VisSpecSize>;
  fragmentSizes: Map<string, VisSpecSize>;
  containerPadding: Map<
    string,
    { left: number; right: number; top: number; bottom: number }
  >;
};

type FragmentBounds = {
  top: number;
  bottom: number;
  left: number;
  width: number;
};

export function renderSequenceView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const layout = layoutSpec(spec, layoutConfig);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
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
  const applyToAll = layoutConfig?.applyToAll ?? false;
  const useSequenceLayout =
    !layoutContext.allowSpecPositions &&
    !applyToAll &&
    layoutContext.requestedStrategy === "auto";
  const sequenceLayout = buildSequenceLayout(
    spec,
    layout,
    labelOptions,
    compartmentMode,
    { enabled: useSequenceLayout },
  );

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
    const padding = sequenceLayout.containerPadding.get(node.id);
    if (padding) {
      element.set("sysmlContainerPadding", padding);
    }
    if (useSequenceLayout) {
      element.set("sysmlMinRect", element.size());
    }
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  fragments.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildFragment(resolvedNode, index, nodeLayout);
    const sizeOverride = sequenceLayout.fragmentSizes.get(node.id);
    if (sizeOverride) {
      element.resize(sizeOverride.width, sizeOverride.height);
    }
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
    if (useSequenceLayout) {
      element.set("sysmlMinRect", element.size());
    }
    graph.addCell(element);
    nodes.set(node.id, element);
  });

  activations.forEach((node, index) => {
    const position = sequenceLayout.positions.get(node.id);
    const resolvedNode = resolveNodeForLayout(node, position, layoutContext);
    const element = buildActivation(resolvedNode, index, nodeLayout, nodes);
    const sizeOverride = sequenceLayout.activationSizes.get(node.id);
    if (sizeOverride) {
      element.resize(sizeOverride.width, sizeOverride.height);
    }
    const overridePos = sequenceLayout.positions.get(node.id);
    if (overridePos && node.parentId) {
      const parent = nodes.get(node.parentId);
      if (parent) {
        const parentBox = parent.getBBox();
        const size = element.size();
        element.position(
          parentBox.x + parentBox.width / 2 - size.width / 2,
          overridePos.y,
        );
      }
    }
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

  if (!useSequenceLayout) {
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
  }
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

  const headerHeight = SEQUENCE_HEADER_HEIGHT;
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
  options?: { enabled?: boolean },
): SequenceLayoutResult {
  const positions = new Map(layout.positions);
  const activationSizes = new Map<string, VisSpecSize>();
  const fragmentSizes = new Map<string, VisSpecSize>();
  const containerPadding = new Map<
    string,
    { left: number; right: number; top: number; bottom: number }
  >();

  if (!options?.enabled || layout.context.allowSpecPositions) {
    return {
      positions,
      lifelineHeight: null,
      activationSizes,
      fragmentSizes,
      containerPadding,
    };
  }

  const nodeById = new Map(spec.nodes.map((node) => [node.id, node]));
  const containers = spec.nodes.filter((node) => isSequenceContainer(node));
  const containerIds = new Set(containers.map((node) => node.id));
  const lifelines = spec.nodes.filter((node) => isLifeline(node));
  const events = spec.nodes.filter((node) => isEventOccurrence(node));
  const activations = spec.nodes.filter((node) => isActivation(node));
  const fragments = spec.nodes.filter((node) => isFragment(node));
  const eventsById = new Map(events.map((node) => [node.id, node]));
  const lifelineOwnerCache = new Map<string, string | null>();

  const lifelinesByContainer = new Map<string | null, VisSpecNode[]>();
  lifelines.forEach((node) => {
    const containerId =
      node.parentId && containerIds.has(node.parentId) ? node.parentId : null;
    const list = lifelinesByContainer.get(containerId) ?? [];
    list.push(node);
    lifelinesByContainer.set(containerId, list);
  });

  if (lifelinesByContainer.size === 0) {
    return {
      positions,
      lifelineHeight: null,
      activationSizes,
      fragmentSizes,
      containerPadding,
    };
  }

  let maxLifelineHeight = 0;

  lifelinesByContainer.forEach((groupLifelines, containerId) => {
    if (groupLifelines.length === 0) {
      return;
    }

    const containerNode = containerId ? nodeById.get(containerId) : undefined;
    const containerPos = containerNode
      ? positions.get(containerNode.id)
      : undefined;
    const baseX = containerPos?.x ?? layout.config.padding.x;
    const baseY = containerPos?.y ?? layout.config.padding.y;
    const textHeight = containerNode
      ? estimateTextCompartmentHeight(
          filterTextCompartments(containerNode.compartments, compartmentMode),
          GRAPHICAL_COMPARTMENT.lineHeight,
          GRAPHICAL_COMPARTMENT.gap,
        )
      : 0;
    const padding = containerNode
      ? resolveSequenceContainerPadding(textHeight)
      : { left: 0, right: 0, top: 0, bottom: 0 };

    if (containerNode) {
      containerPadding.set(containerNode.id, padding);
    }

    const originX = baseX + padding.left;
    const originY = baseY + padding.top;

    const widths = groupLifelines.map((node) => {
      const estimated = estimateNodeSize(
        node,
        layout.config.defaultSize,
        labelOptions,
        compartmentMode,
      ).width;
      return Math.max(estimated, node.size?.width ?? 0);
    });
    const maxWidth = Math.max(layout.config.defaultSize.width, ...widths);
    const lifelineGap = Math.max(layout.config.layerGap, 100);

    groupLifelines.forEach((node, index) => {
      positions.set(node.id, {
        x: originX + index * (maxWidth + lifelineGap),
        y: originY,
      });
    });

    const lifelineIds = new Set(groupLifelines.map((node) => node.id));
    const eventsInGroup = events.filter(
      (node) => node.parentId && lifelineIds.has(node.parentId),
    );
    const eventIds = new Set(eventsInGroup.map((node) => node.id));
    const rowGap = Math.max(36, Math.round(layout.config.nodeGap));
    const eventOffset = Math.max(16, Math.round(rowGap * 0.4));
    const eventStartY = originY + SEQUENCE_HEADER_HEIGHT + eventOffset;
    const eventY = new Map<string, number>();
    let cursorY = eventStartY;

    spec.links.forEach((link) => {
      const source = eventsById.get(link.source.nodeId);
      const target = eventsById.get(link.target.nodeId);
      if (!source || !target) {
        return;
      }
      if (!eventIds.has(source.id) || !eventIds.has(target.id)) {
        return;
      }

      const sameLifeline = source.parentId === target.parentId;
      const isSuccession = isSuccessionLink(link.kind);
      if (sameLifeline || isSuccession) {
        cursorY = assignVerticalEventRow(
          eventY,
          source.id,
          target.id,
          cursorY,
          eventStartY,
          rowGap,
        );
      } else {
        cursorY = assignHorizontalEventRow(
          eventY,
          source.id,
          target.id,
          cursorY,
          rowGap,
        );
      }
    });

    eventsInGroup.forEach((node) => {
      if (eventY.has(node.id)) {
        return;
      }
      eventY.set(node.id, cursorY);
      cursorY += rowGap;
    });

    let maxEventY = eventStartY;
    eventY.forEach((y, id) => {
      positions.set(id, { x: 0, y });
      maxEventY = Math.max(maxEventY, y);
    });

    const eventBottom = maxEventY + EVENT_DIAMETER + Math.round(rowGap * 0.6);
    const lifelineHeight = Math.max(
      layout.config.defaultSize.height,
      eventBottom - originY,
    );
    maxLifelineHeight = Math.max(maxLifelineHeight, lifelineHeight);

    const eventYByLifeline = new Map<string, number[]>();
    eventY.forEach((y, id) => {
      const parentId = eventsById.get(id)?.parentId;
      if (!parentId) {
        return;
      }
      const list = eventYByLifeline.get(parentId) ?? [];
      list.push(y);
      eventYByLifeline.set(parentId, list);
    });

    const activationOwners = new Map<string, string>();
    const activationsInGroup = activations.filter((node) => {
      const owner = resolveLifelineOwner(node, nodeById, lifelineOwnerCache);
      if (owner && lifelineIds.has(owner)) {
        activationOwners.set(node.id, owner);
        return true;
      }
      return false;
    });
    const activationLookup = new Map(
      activationsInGroup.map((node) => [node.id, node]),
    );
    const activationDepthCache = new Map<string, number>();
    const activationBounds = new Map<string, { top: number; bottom: number }>();
    const sortedActivations = activationsInGroup
      .slice()
      .sort(
        (left, right) =>
          resolveActivationDepth(
            left.id,
            activationLookup,
            activationDepthCache,
          ) -
          resolveActivationDepth(
            right.id,
            activationLookup,
            activationDepthCache,
          ),
      );

    sortedActivations.forEach((node) => {
      const lifelineOwner = activationOwners.get(node.id);
      if (!lifelineOwner) {
        return;
      }
      const lifelineEvents = eventYByLifeline.get(lifelineOwner) ?? [];
      let startY =
        lifelineEvents.length > 0 ? Math.min(...lifelineEvents) : eventStartY;
      let endY =
        lifelineEvents.length > 0
          ? Math.max(...lifelineEvents) + EVENT_DIAMETER
          : startY + rowGap;
      const parentBounds = node.parentId && activationBounds.get(node.parentId);
      if (parentBounds) {
        const innerTop = parentBounds.top + NESTED_ACTIVATION_PADDING;
        const innerBottom = parentBounds.bottom - NESTED_ACTIVATION_PADDING;
        startY = clamp(startY, innerTop, innerBottom);
        endY = clamp(
          endY,
          startY + Math.max(12, Math.round(rowGap * 0.3)),
          innerBottom,
        );
      }
      const width = node.size?.width ?? 12;
      const height = Math.max(
        node.size?.height ?? 70,
        endY - startY + Math.round(rowGap * 0.2),
      );
      positions.set(node.id, { x: 0, y: startY });
      activationSizes.set(node.id, { width, height });
      activationBounds.set(node.id, { top: startY, bottom: startY + height });
    });

    const containerKey = containerId ?? null;
    const fragmentsInGroup = fragments.filter((node) => {
      const ancestor = resolveFragmentContainer(node, containerIds, nodeById);
      return ancestor === containerId;
    });
    if (fragmentsInGroup.length > 0) {
      const firstLifeline = groupLifelines[0];
      const lastLifeline = groupLifelines[groupLifelines.length - 1];
      const firstPos = positions.get(firstLifeline.id);
      const lastPos = positions.get(lastLifeline.id);
      const firstX = firstPos?.x ?? originX;
      const lastX = lastPos?.x ?? originX;
      const spanWidth = lastX - firstX + maxWidth;
      const width = Math.max(
        layout.config.defaultSize.width,
        spanWidth + FRAGMENT_PADDING * 2,
      );
      const timelineTop = eventStartY - Math.round(rowGap * 0.6);
      const timelineBottom = Math.max(timelineTop + 140, maxEventY + rowGap);
      const fragmentsByParent = buildFragmentHierarchy(
        fragmentsInGroup,
        containerKey,
      );
      const fragmentGap = Math.max(24, Math.round(rowGap * 0.6));
      layoutFragmentsForParent(
        containerKey,
        {
          top: timelineTop,
          bottom: timelineBottom,
          left: firstX - FRAGMENT_PADDING,
          width,
        },
        fragmentsByParent,
        fragmentSizes,
        positions,
        fragmentGap,
      );
    }
  });

  return {
    positions,
    lifelineHeight: maxLifelineHeight > 0 ? maxLifelineHeight : null,
    activationSizes,
    fragmentSizes,
    containerPadding,
  };
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

function resolveSequenceContainerPadding(textHeight: number): {
  left: number;
  right: number;
  top: number;
  bottom: number;
} {
  const side = GRAPHICAL_COMPARTMENT.padding;
  return {
    left: side,
    right: side,
    top: GRAPHICAL_COMPARTMENT.headerHeight + textHeight + side,
    bottom: side,
  };
}

function estimateTextCompartmentHeight(
  compartments: VisSpecCompartment[] | undefined,
  lineHeight: number,
  compartmentGap: number,
): number {
  if (!compartments) {
    return 0;
  }

  let total = 0;
  compartments
    .filter((compartment) => compartment.kind === "text")
    .forEach((compartment) => {
      const titleLines = compartment.title ? 1 : 0;
      const contentLines = compartment.lines ? compartment.lines.length : 0;
      const lines = titleLines + contentLines;
      if (lines === 0) {
        return;
      }
      total += lines * lineHeight + compartmentGap;
    });

  return total;
}

function isSuccessionLink(kind: string): boolean {
  const normalized = kind.toLowerCase();
  return (
    normalized.includes("succession") ||
    normalized.includes("occurrence") ||
    normalized.includes("next")
  );
}

function assignHorizontalEventRow(
  eventY: Map<string, number>,
  sourceId: string,
  targetId: string,
  cursorY: number,
  rowGap: number,
): number {
  const sourceY = eventY.get(sourceId);
  const targetY = eventY.get(targetId);
  const resolvedY = sourceY ?? targetY ?? cursorY;

  if (sourceY === undefined) {
    eventY.set(sourceId, resolvedY);
  }
  if (targetY === undefined) {
    eventY.set(targetId, resolvedY);
  }

  return Math.max(cursorY, resolvedY + rowGap);
}

function assignVerticalEventRow(
  eventY: Map<string, number>,
  sourceId: string,
  targetId: string,
  cursorY: number,
  startY: number,
  rowGap: number,
): number {
  const sourceY = eventY.get(sourceId);
  const targetY = eventY.get(targetId);

  if (sourceY !== undefined && targetY !== undefined) {
    return Math.max(cursorY, Math.max(sourceY, targetY) + rowGap);
  }

  if (sourceY !== undefined) {
    const nextY = Math.max(cursorY, sourceY + rowGap);
    eventY.set(targetId, nextY);
    return nextY + rowGap;
  }

  if (targetY !== undefined) {
    const prevY = Math.max(startY, targetY - rowGap);
    eventY.set(sourceId, prevY);
    return Math.max(cursorY, targetY + rowGap);
  }

  eventY.set(sourceId, cursorY);
  eventY.set(targetId, cursorY + rowGap);
  return cursorY + rowGap * 2;
}

function resolveFragmentContainer(
  node: VisSpecNode,
  containerIds: Set<string>,
  nodeById: Map<string, VisSpecNode>,
): string | null {
  let current: VisSpecNode | undefined = node;
  while (current?.parentId) {
    if (containerIds.has(current.parentId)) {
      return current.parentId;
    }
    current = nodeById.get(current.parentId);
  }
  return null;
}

function buildFragmentHierarchy(
  fragments: VisSpecNode[],
  containerKey: string | null,
): Map<string | null, VisSpecNode[]> {
  const fragmentIds = new Set(fragments.map((fragment) => fragment.id));
  const hierarchy = new Map<string | null, VisSpecNode[]>();
  fragments.forEach((fragment) => {
    const parentKey =
      fragment.parentId && fragmentIds.has(fragment.parentId)
        ? fragment.parentId
        : containerKey;
    const group = hierarchy.get(parentKey) ?? [];
    group.push(fragment);
    hierarchy.set(parentKey, group);
  });
  return hierarchy;
}

function layoutFragmentsForParent(
  parentKey: string | null,
  bounds: FragmentBounds,
  fragmentsByParent: Map<string | null, VisSpecNode[]>,
  fragmentSizes: Map<string, VisSpecSize>,
  positions: Map<string, VisSpecPoint>,
  gap: number,
): void {
  const children = fragmentsByParent.get(parentKey);
  if (!children || children.length === 0) {
    return;
  }
  const availableHeight = Math.max(
    bounds.bottom - bounds.top,
    MIN_FRAGMENT_HEIGHT,
  );
  const width = Math.max(bounds.width, MIN_FRAGMENT_WIDTH);
  const gapTotal = gap * (children.length - 1);
  const height = Math.max(
    MIN_FRAGMENT_HEIGHT,
    (availableHeight - gapTotal) / children.length,
  );

  children.forEach((node, index) => {
    const y = bounds.top + index * (height + gap);
    positions.set(node.id, { x: bounds.left, y });
    fragmentSizes.set(node.id, { width, height });
    const innerBounds: FragmentBounds = {
      top: y + FRAGMENT_HEADER_HEIGHT,
      bottom: Math.max(
        y + height - FRAGMENT_PADDING,
        y + FRAGMENT_HEADER_HEIGHT + MIN_FRAGMENT_HEIGHT / 2,
      ),
      left: bounds.left + FRAGMENT_PADDING,
      width: Math.max(width - FRAGMENT_PADDING * 2, MIN_FRAGMENT_WIDTH),
    };
    layoutFragmentsForParent(
      node.id,
      innerBounds,
      fragmentsByParent,
      fragmentSizes,
      positions,
      gap,
    );
  });
}

function resolveLifelineOwner(
  node: VisSpecNode,
  nodeById: Map<string, VisSpecNode>,
  cache: Map<string, string | null>,
): string | null {
  if (cache.has(node.id)) {
    return cache.get(node.id) ?? null;
  }

  let current: VisSpecNode | undefined = node;
  while (current?.parentId) {
    const parent = nodeById.get(current.parentId);
    if (!parent) {
      break;
    }
    if (isLifeline(parent)) {
      cache.set(node.id, parent.id);
      return parent.id;
    }
    current = parent;
  }

  cache.set(node.id, null);
  return null;
}

function resolveActivationDepth(
  id: string,
  lookup: Map<string, VisSpecNode>,
  cache: Map<string, number>,
): number {
  if (cache.has(id)) {
    return cache.get(id) ?? 0;
  }
  const node = lookup.get(id);
  if (!node || !node.parentId || !lookup.has(node.parentId)) {
    cache.set(id, 0);
    return 0;
  }
  const depth = resolveActivationDepth(node.parentId, lookup, cache) + 1;
  cache.set(id, depth);
  return depth;
}

function clamp(value: number, min: number, max: number): number {
  if (Number.isNaN(value)) {
    return min;
  }
  if (max < min) {
    return min;
  }
  if (value < min) {
    return min;
  }
  if (value > max) {
    return max;
  }
  return value;
}
