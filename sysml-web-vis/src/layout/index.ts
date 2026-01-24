import { dia } from "@joint/core";
import { applyExternalLayout } from "./external";
import type {
  VisSpec,
  VisSpecLink,
  VisSpecNode,
  VisSpecPoint,
  VisSpecSize,
  VisSpecView,
} from "../vis-spec";
import {
  formatIconKindLabel,
  formatNodeLabel,
  LabelingOptions,
  resolveLabelingOptions,
} from "../notation";
import {
  CompartmentMode,
  filterTextCompartments,
  resolveCompartmentMode,
} from "../compartments";

export type LayoutStrategy =
  | "auto"
  | "interconnection"
  | "manual"
  | "grid"
  | "layered"
  | "dagre"
  | "msagl"
  | "elk";
export type ExternalLayoutStrategy = "dagre" | "msagl" | "elk";
type InternalLayoutStrategy = "manual" | "grid" | "layered";
export type LayoutDirection = "LR" | "TB";

const EXTERNAL_LAYOUTS = new Set<ExternalLayoutStrategy>([
  "dagre",
  "msagl",
  "elk",
]);

export interface LayoutConfig {
  strategy?: LayoutStrategy;
  direction?: LayoutDirection;
  defaultSize?: VisSpecSize;
  padding?: VisSpecPoint;
  perRow?: number;
  layerGap?: number;
  nodeGap?: number;
  applyToAll?: boolean;
}

export interface ResolvedLayoutConfig {
  strategy: InternalLayoutStrategy;
  direction: LayoutDirection;
  defaultSize: VisSpecSize;
  padding: VisSpecPoint;
  perRow: number;
  layerGap: number;
  nodeGap: number;
  applyToAll: boolean;
}

export interface LayoutResult {
  config: ResolvedLayoutConfig;
  positions: Map<string, VisSpecPoint>;
  context: LayoutContext;
}

export interface ApplyLayoutOptions {
  graph: dia.Graph;
  spec: VisSpec;
  render: () => void | Promise<void>;
  paper?: dia.Paper;
  layoutConfig?: LayoutConfig;
}

export function isExternalLayoutStrategy(
  strategy?: LayoutStrategy | null,
): strategy is ExternalLayoutStrategy {
  if (!strategy) {
    return false;
  }
  return EXTERNAL_LAYOUTS.has(strategy as ExternalLayoutStrategy);
}

export async function applyLayout({
  graph,
  spec,
  render,
  paper,
  layoutConfig,
}: ApplyLayoutOptions): Promise<ResolvedLayoutConfig> {
  const resolved = resolveLayoutConfig(spec, layoutConfig);
  await render();

  const strategy = layoutConfig?.strategy;
  if (isExternalLayoutStrategy(strategy)) {
    if (!paper) {
      console.warn(
        "[sysml-web-vis] External layout requested but no paper provided.",
      );
    } else {
      await applyExternalLayout(graph, paper, strategy, resolved.direction);
    }
  }

  return resolved;
}

export interface LayoutContext {
  requestedStrategy: LayoutStrategy;
  isExternal: boolean;
  allowManualPosition: boolean;
  allowManualSize: boolean;
  allowSpecPositions: boolean;
  allowSpecSizes: boolean;
}

const VIEW_DEFAULTS: Record<VisSpecView, ResolvedLayoutConfig> = {
  GeneralView: {
    strategy: "layered",
    direction: "TB",
    defaultSize: { width: 180, height: 64 },
    padding: { x: 20, y: 20 },
    perRow: 3,
    layerGap: 50,
    nodeGap: 50,
  },
  InterconnectionView: {
    strategy: "grid",
    direction: "LR",
    defaultSize: { width: 220, height: 90 },
    padding: { x: 60, y: 60 },
    perRow: 3,
    layerGap: 140,
    nodeGap: 90,
  },
  StateTransitionView: {
    strategy: "layered",
    direction: "TB",
    defaultSize: { width: 190, height: 72 },
    padding: { x: 60, y: 60 },
    perRow: 2,
    layerGap: 50,
    nodeGap: 50,
  },
  ActionFlowView: {
    strategy: "layered",
    direction: "TB",
    defaultSize: { width: 200, height: 72 },
    padding: { x: 10, y: 10 },
    perRow: 2,
    layerGap: 50,
    nodeGap: 50,
  },
  SequenceView: {
    strategy: "grid",
    direction: "LR",
    defaultSize: { width: 160, height: 260 },
    padding: { x: 50, y: 50 },
    perRow: 3,
    layerGap: 50,
    nodeGap: 50,
  },
  BrowserView: {
    strategy: "layered",
    direction: "TB",
    defaultSize: { width: 240, height: 48 },
    padding: { x: 40, y: 40 },
    perRow: 1,
    layerGap: 50,
    nodeGap: 50,
  },
  GridView: {
    strategy: "grid",
    direction: "LR",
    defaultSize: { width: 180, height: 64 },
    padding: { x: 50, y: 50 },
    perRow: 3,
    layerGap: 50,
    nodeGap: 50,
  },
  GeometryView: {
    strategy: "manual",
    direction: "LR",
    defaultSize: { width: 180, height: 80 },
    padding: { x: 80, y: 80 },
    perRow: 2,
    layerGap: 50,
    nodeGap: 50,
  },
};

export function layoutSpec(spec: VisSpec, config?: LayoutConfig): LayoutResult {
  const resolved = resolveLayoutConfig(spec, config);
  const context = resolveLayoutContext(spec, config, resolved);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
  const orderedNodes = resolveLayoutNodes(spec);
  const sizingNodes = context.allowSpecSizes
    ? orderedNodes
    : orderedNodes.map((node) =>
        node.size ? { ...node, size: undefined } : node,
      );
  const rootNodes = orderedNodes.filter((node) => !node.parentId);
  const sizeById = new Map(
    sizingNodes.map((node) => [
      node.id,
      estimateNodeSize(
        node,
        resolved.defaultSize,
        labelOptions,
        compartmentMode,
      ),
    ]),
  );

  if (resolved.strategy === "manual") {
    return {
      config: resolved,
      positions: new Map(
        spec.nodes
          .filter((node) => node.position)
          .map((node) => [node.id, node.position as VisSpecPoint]),
      ),
      context,
    };
  }

  const layoutNodes = resolved.applyToAll
    ? orderedNodes
    : rootNodes.length > 0
      ? rootNodes
      : orderedNodes;
  const basePositions =
    resolved.strategy === "grid"
      ? gridLayout(layoutNodes, resolved, sizeById)
      : layeredLayout(layoutNodes, spec.links, resolved, sizeById);

  const positions = new Map(basePositions);

  const separated = separateOverlaps(
    spec,
    positions,
    sizeById,
    Math.max(24, Math.min(resolved.nodeGap, resolved.layerGap) / 2),
  );

  return { config: resolved, positions: separated, context };
}

export function resolveLayoutContext(
  spec: VisSpec,
  config?: LayoutConfig,
  resolved?: ResolvedLayoutConfig,
): LayoutContext {
  const requestedStrategy = config?.strategy ?? "auto";
  const resolvedConfig = resolved ?? resolveLayoutConfig(spec, config);
  const isExternal = isExternalLayoutStrategy(requestedStrategy);
  const allowManualPosition = resolvedConfig.strategy === "manual";
  const allowManualSize = resolvedConfig.strategy === "manual";

  return {
    requestedStrategy,
    isExternal,
    allowManualPosition,
    allowManualSize,
    allowSpecPositions: allowManualPosition,
    allowSpecSizes: allowManualSize,
  };
}

export function resolveLayoutConfig(
  spec: VisSpec,
  config?: LayoutConfig,
): ResolvedLayoutConfig {
  const defaults = VIEW_DEFAULTS[spec.view];
  const strategy = config?.strategy ?? "auto";
  const externalStrategies = new Set<LayoutStrategy>(["dagre", "msagl", "elk"]);

  let resolvedStrategy: ResolvedLayoutConfig["strategy"];
  if (strategy === "auto") {
    resolvedStrategy = defaults.strategy;
  } else if (strategy === "interconnection") {
    resolvedStrategy = defaults.strategy;
  } else if (externalStrategies.has(strategy)) {
    resolvedStrategy = defaults.strategy;
  } else {
    resolvedStrategy = strategy;
  }

  return {
    strategy: resolvedStrategy,
    direction: config?.direction ?? defaults.direction,
    defaultSize: config?.defaultSize ?? defaults.defaultSize,
    padding: config?.padding ?? defaults.padding,
    perRow: config?.perRow ?? defaults.perRow,
    layerGap: config?.layerGap ?? defaults.layerGap,
    nodeGap: config?.nodeGap ?? defaults.nodeGap,
    applyToAll: config?.applyToAll ?? false,
  };
}

function resolveLayoutNodes(spec: VisSpec): VisSpecNode[] {
  const nodesWithIndex = spec.nodes.map((node, index) => ({ node, index }));
  const sortBy = spec.viewMetadata?.sortBy;
  const sortOrder = spec.viewMetadata?.sortOrder ?? "asc";
  const direction = sortOrder === "desc" ? -1 : 1;

  if (sortBy === "name" || sortBy === "kind") {
    nodesWithIndex.sort((left, right) => {
      const leftValue =
        sortBy === "name" ? resolveNodeName(left.node) : (left.node.kind ?? "");
      const rightValue =
        sortBy === "name"
          ? resolveNodeName(right.node)
          : (right.node.kind ?? "");
      const text = compareText(leftValue, rightValue);
      if (text !== 0) {
        return text * direction;
      }
      return left.index - right.index;
    });
  } else if (spec.view === "GeneralView") {
    nodesWithIndex.sort((left, right) => {
      const groupDelta =
        resolveGeneralKindGroup(left.node.kind) -
        resolveGeneralKindGroup(right.node.kind);
      if (groupDelta !== 0) {
        return groupDelta;
      }
      const kindDelta = compareText(
        left.node.kind ?? "",
        right.node.kind ?? "",
      );
      if (kindDelta !== 0) {
        return kindDelta;
      }
      const nameDelta = compareText(
        resolveNodeName(left.node),
        resolveNodeName(right.node),
      );
      if (nameDelta !== 0) {
        return nameDelta;
      }
      return left.index - right.index;
    });
  }

  return nodesWithIndex.map(({ node }) => node);
}

function resolveNodeName(node: VisSpecNode): string {
  return node.name ?? node.label ?? node.id ?? "";
}

function compareText(left: string, right: string): number {
  return left.localeCompare(right, undefined, { sensitivity: "base" });
}

function resolveGeneralKindGroup(kind: string): number {
  const normalized = kind.toLowerCase();
  if (normalized.includes("package")) {
    return 0;
  }
  if (normalized.includes("viewpoint") || normalized.includes("view")) {
    return 1;
  }
  if (normalized.includes("concern") || normalized.includes("stakeholder")) {
    return 2;
  }
  if (normalized.includes("requirement")) {
    return 3;
  }
  if (normalized.includes("part") || normalized.includes("block")) {
    return 4;
  }
  if (normalized.includes("connection") || normalized.includes("interface")) {
    return 5;
  }
  if (normalized.includes("constraint") || normalized.includes("parameter")) {
    return 6;
  }
  if (normalized.includes("action") || normalized.includes("activity")) {
    return 7;
  }
  if (normalized.includes("usecase") || normalized.includes("use case")) {
    return 8;
  }
  if (normalized.includes("state")) {
    return 9;
  }
  if (normalized.includes("actor")) {
    return 10;
  }
  return 11;
}

function gridLayout(
  nodes: VisSpecNode[],
  config: ResolvedLayoutConfig,
  sizeById: Map<string, VisSpecSize>,
): Map<string, VisSpecPoint> {
  const positions = new Map<string, VisSpecPoint>();
  const totalRows = Math.ceil(nodes.length / config.perRow);
  const columnWidths = new Array(config.perRow).fill(0);
  const rowHeights = new Array(totalRows).fill(0);

  nodes.forEach((node, index) => {
    const row = Math.floor(index / config.perRow);
    const col = index % config.perRow;
    const size = sizeById.get(node.id) ?? config.defaultSize;
    columnWidths[col] = Math.max(columnWidths[col], size.width);
    rowHeights[row] = Math.max(rowHeights[row], size.height);
  });

  const columnOffsets: number[] = [];
  let cursorX = config.padding.x;
  columnWidths.forEach((width, index) => {
    columnOffsets[index] = cursorX;
    cursorX += width + config.layerGap;
  });

  const rowOffsets: number[] = [];
  let cursorY = config.padding.y;
  rowHeights.forEach((height, index) => {
    rowOffsets[index] = cursorY;
    cursorY += height + config.nodeGap;
  });

  nodes.forEach((node, index) => {
    const row = Math.floor(index / config.perRow);
    const col = index % config.perRow;
    positions.set(node.id, {
      x: columnOffsets[col] ?? config.padding.x,
      y: rowOffsets[row] ?? config.padding.y,
    });
  });

  return positions;
}

export function estimateNodeSize(
  node: VisSpecNode,
  defaultSize: VisSpecSize,
  labelOptions?: LabelingOptions,
  compartmentMode: CompartmentMode = "mixed",
): VisSpecSize {
  if (node.size) {
    return node.size;
  }

  const minSize = resolveMinimumSize(node, defaultSize);
  const label = formatNodeLabel(node, labelOptions);
  const iconLabel = formatIconKindLabel(node, labelOptions);
  const detailLines = collectTextLines(
    filterTextCompartments(node.compartments, compartmentMode),
  );

  const maxLineLength = Math.max(
    iconLabel.length,
    label.length,
    ...detailLines.map((line) => line.length),
  );

  const approxCharWidth = 7;
  const padding = 36;
  const maxWidth = defaultSize.width * 1.6;
  const width = Math.max(
    minSize.width,
    defaultSize.width,
    Math.min(maxWidth, maxLineLength * approxCharWidth + padding),
  );

  let height = defaultSize.height;
  if (detailLines.length > 0) {
    height += detailLines.length * 14 + 12;
  }

  return { width, height: Math.max(height, minSize.height) };
}

function resolveMinimumSize(
  node: VisSpecNode,
  fallback: VisSpecSize,
): VisSpecSize {
  const kind = node.kind.toLowerCase();

  if (kind.includes("fork") || kind.includes("join")) {
    return { width: 60, height: 10 };
  }
  if (kind.includes("decision") || kind.includes("merge")) {
    return { width: 46, height: 46 };
  }
  if (kind.includes("initial") || kind.includes("start")) {
    return { width: 22, height: 22 };
  }
  if (kind.includes("final") || kind.includes("terminate")) {
    return { width: 24, height: 24 };
  }
  if (kind.includes("activation") || kind.includes("execution")) {
    return { width: 12, height: 70 };
  }
  if (
    kind.includes("event") ||
    kind.includes("occurrence") ||
    kind.includes("messagepoint")
  ) {
    return { width: 14, height: 14 };
  }
  if (kind.includes("lifeline")) {
    return { width: 160, height: 260 };
  }
  if (kind.includes("sequence") || kind.includes("interaction")) {
    return { width: 280, height: 140 };
  }
  if (kind.includes("actor")) {
    return { width: 80, height: 120 };
  }
  if (kind.includes("usecase") || kind.includes("use case")) {
    return { width: 160, height: 70 };
  }
  if (kind.includes("requirement")) {
    return { width: 190, height: 72 };
  }
  if (kind.includes("constraint")) {
    return { width: 180, height: 64 };
  }
  if (kind.includes("state")) {
    return { width: 190, height: 72 };
  }
  if (kind.includes("parameter") || kind.includes("pin")) {
    return { width: 140, height: 44 };
  }
  if (
    kind.includes("trigger") ||
    kind.includes("time") ||
    kind.includes("change")
  ) {
    return { width: 150, height: 44 };
  }
  if (kind.includes("action")) {
    return { width: 180, height: 64 };
  }

  return fallback;
}

function collectTextLines(compartments: VisSpecNode["compartments"]): string[] {
  if (!compartments) {
    return [];
  }

  const lines: string[] = [];
  compartments
    .filter((compartment) => compartment.kind === "text")
    .forEach((compartment) => {
      if (compartment.title) {
        lines.push(compartment.title);
      }
      if (compartment.lines && compartment.lines.length > 0) {
        compartment.lines.forEach((line) => lines.push(line));
      }
    });

  return lines;
}

function layeredLayout(
  nodes: VisSpecNode[],
  links: VisSpecLink[],
  config: ResolvedLayoutConfig,
  sizeById: Map<string, VisSpecSize>,
): Map<string, VisSpecPoint> {
  const positions = new Map<string, VisSpecPoint>();
  const nodeIds = nodes.map((node) => node.id);
  const indexById = new Map(nodeIds.map((id, index) => [id, index]));
  const outgoing = new Map<string, string[]>();
  const indegree = new Map<string, number>();

  nodeIds.forEach((id) => {
    outgoing.set(id, []);
    indegree.set(id, 0);
  });

  links.forEach((link) => {
    if (
      !outgoing.has(link.source.nodeId) ||
      !outgoing.has(link.target.nodeId)
    ) {
      return;
    }

    outgoing.get(link.source.nodeId)?.push(link.target.nodeId);
    indegree.set(
      link.target.nodeId,
      (indegree.get(link.target.nodeId) ?? 0) + 1,
    );
  });

  const rank = new Map<string, number>();
  const queue: string[] = nodeIds.filter((id) => (indegree.get(id) ?? 0) === 0);

  while (queue.length > 0) {
    const id = queue.shift();
    if (!id) {
      continue;
    }

    const currentRank = rank.get(id) ?? 0;
    const targets = outgoing.get(id) ?? [];

    targets.forEach((target) => {
      const nextRank = currentRank + 1;
      const existingRank = rank.get(target) ?? 0;
      rank.set(target, Math.max(existingRank, nextRank));

      const remaining = (indegree.get(target) ?? 0) - 1;
      indegree.set(target, remaining);
      if (remaining === 0) {
        queue.push(target);
      }
    });
  }

  nodeIds.forEach((id) => {
    if (!rank.has(id)) {
      rank.set(id, 0);
    }
  });

  const layers = new Map<number, string[]>();
  nodeIds.forEach((id) => {
    const level = rank.get(id) ?? 0;
    const bucket = layers.get(level) ?? [];
    bucket.push(id);
    layers.set(level, bucket);
  });

  const sortedLayers = Array.from(layers.entries()).sort(([a], [b]) => a - b);

  if (config.direction === "LR") {
    let cursorX = config.padding.x;
    sortedLayers.forEach(([, ids]) => {
      const sortedIds = ids.sort(
        (left, right) =>
          (indexById.get(left) ?? 0) - (indexById.get(right) ?? 0),
      );
      const layerSize = sortedIds.reduce(
        (acc, id) => {
          const size = sizeById.get(id) ?? config.defaultSize;
          return {
            width: Math.max(acc.width, size.width),
            height: Math.max(acc.height, size.height),
          };
        },
        { width: 0, height: 0 },
      );

      sortedIds.forEach((id, index) => {
        positions.set(id, {
          x: cursorX,
          y: config.padding.y + index * (layerSize.height + config.nodeGap),
        });
      });

      cursorX += layerSize.width + config.layerGap;
    });
  } else {
    let cursorY = config.padding.y;
    sortedLayers.forEach(([, ids]) => {
      const sortedIds = ids.sort(
        (left, right) =>
          (indexById.get(left) ?? 0) - (indexById.get(right) ?? 0),
      );
      const layerSize = sortedIds.reduce(
        (acc, id) => {
          const size = sizeById.get(id) ?? config.defaultSize;
          return {
            width: Math.max(acc.width, size.width),
            height: Math.max(acc.height, size.height),
          };
        },
        { width: 0, height: 0 },
      );

      sortedIds.forEach((id, index) => {
        positions.set(id, {
          x: config.padding.x + index * (layerSize.width + config.nodeGap),
          y: cursorY,
        });
      });

      cursorY += layerSize.height + config.layerGap;
    });
  }

  return positions;
}

function separateOverlaps(
  spec: VisSpec,
  positions: Map<string, VisSpecPoint>,
  sizeById: Map<string, VisSpecSize>,
  gap: number,
): Map<string, VisSpecPoint> {
  if (positions.size === 0) {
    return positions;
  }

  const adjusted = new Map(positions);
  const groups = groupNodesByParent(spec.nodes);
  const childrenByParent = buildChildrenByParent(spec.nodes);

  groups.forEach((groupIds) => {
    if (groupIds.length <= 1) {
      return;
    }

    const maxIterations = Math.max(4, groupIds.length);

    for (let iteration = 0; iteration < maxIterations; iteration += 1) {
      let moved = false;

      for (let i = 0; i < groupIds.length; i += 1) {
        const idA = groupIds[i];
        const posA = adjusted.get(idA);
        if (!posA) {
          continue;
        }
        const sizeA = sizeById.get(idA);
        if (!sizeA) {
          continue;
        }

        for (let j = i + 1; j < groupIds.length; j += 1) {
          const idB = groupIds[j];
          const posB = adjusted.get(idB);
          const sizeB = sizeById.get(idB);
          if (!posB || !sizeB) {
            continue;
          }

          const overlap = measureOverlap(posA, sizeA, posB, sizeB, gap);
          if (!overlap) {
            continue;
          }

          const dx = overlap.axis === "x" ? overlap.amount : 0;
          const dy = overlap.axis === "y" ? overlap.amount : 0;
          translateSubtree(idB, dx, dy, adjusted, childrenByParent);
          moved = true;
        }
      }

      if (!moved) {
        break;
      }
    }
  });

  return adjusted;
}

function groupNodesByParent(nodes: VisSpecNode[]): Map<string, string[]> {
  const groups = new Map<string, string[]>();

  nodes.forEach((node) => {
    const key = node.parentId ?? "__root__";
    const group = groups.get(key) ?? [];
    group.push(node.id);
    groups.set(key, group);
  });

  return groups;
}

function buildChildrenByParent(nodes: VisSpecNode[]): Map<string, string[]> {
  const children = new Map<string, string[]>();

  nodes.forEach((node) => {
    if (!node.parentId) {
      return;
    }
    const group = children.get(node.parentId) ?? [];
    group.push(node.id);
    children.set(node.parentId, group);
  });

  return children;
}

function translateSubtree(
  rootId: string,
  dx: number,
  dy: number,
  positions: Map<string, VisSpecPoint>,
  childrenByParent: Map<string, string[]>,
): void {
  const current = positions.get(rootId);
  if (current) {
    positions.set(rootId, { x: current.x + dx, y: current.y + dy });
  }

  const children = childrenByParent.get(rootId) ?? [];
  children.forEach((childId) => {
    translateSubtree(childId, dx, dy, positions, childrenByParent);
  });
}

function measureOverlap(
  posA: VisSpecPoint,
  sizeA: VisSpecSize,
  posB: VisSpecPoint,
  sizeB: VisSpecSize,
  gap: number,
): { axis: "x" | "y"; amount: number } | null {
  const centerAx = posA.x + sizeA.width / 2;
  const centerAy = posA.y + sizeA.height / 2;
  const centerBx = posB.x + sizeB.width / 2;
  const centerBy = posB.y + sizeB.height / 2;

  const dx = Math.abs(centerAx - centerBx);
  const dy = Math.abs(centerAy - centerBy);
  const overlapX = sizeA.width / 2 + sizeB.width / 2 + gap - dx;
  const overlapY = sizeA.height / 2 + sizeB.height / 2 + gap - dy;

  if (overlapX <= 0 || overlapY <= 0) {
    return null;
  }

  if (overlapX < overlapY) {
    return { axis: "x", amount: overlapX };
  }

  return { axis: "y", amount: overlapY };
}
