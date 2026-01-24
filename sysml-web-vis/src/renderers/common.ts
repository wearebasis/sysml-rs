import { dia } from "@joint/core";
import { CONTAINER_PADDING, GRAPHICAL_COMPARTMENT } from "../config/constants";
import { fitElementToChildren } from "../auto-fit";
import { estimateNodeSize, LayoutContext } from "../layout";
import {
  SysmlActor,
  SysmlBlock,
  SysmlConstraint,
  SysmlAction,
  SysmlNode,
  SysmlPackage,
  SysmlParameter,
  SysmlRequirement,
  SysmlState,
  SysmlUseCase,
} from "../shapes";
import {
  formatIconKindLabel,
  formatNodeLabel,
  LabelingOptions,
  resolveDefinitionUsageRole,
  shouldApplyDefinitionUsageVariant,
} from "../notation";
import {
  CompartmentMode,
  filterGraphicalCompartments,
  filterTextCompartments,
} from "../compartments";
import {
  VisSpec,
  VisSpecCompartment,
  VisSpecNode,
  VisSpecPoint,
  VisSpecPort,
  VisSpecSize,
} from "../vis-spec";

export enum ZOrder {
  Container = 0,
  Link = 10,
  Node = 20,
}

export interface LayoutOptions {
  defaultSize: VisSpecSize;
  padding: VisSpecPoint;
  gap: VisSpecPoint;
  perRow: number;
}

export interface BuildNodeOptions extends Partial<LayoutOptions> {
  containerIds?: Set<string>;
  labelOptions?: LabelingOptions;
  compartmentMode?: CompartmentMode;
  portLabelOffset?: number;
}

const DEFAULT_LAYOUT: LayoutOptions = {
  defaultSize: { width: 180, height: 64 },
  padding: { x: 40, y: 40 },
  gap: { x: 220, y: 120 },
  perRow: 3,
};

export function buildNode(
  node: VisSpecNode,
  index: number,
  options?: BuildNodeOptions,
): dia.Element {
  const layout: LayoutOptions = {
    defaultSize: {
      ...DEFAULT_LAYOUT.defaultSize,
      ...(options?.defaultSize ?? {}),
    },
    padding: {
      ...DEFAULT_LAYOUT.padding,
      ...(options?.padding ?? {}),
    },
    gap: {
      ...DEFAULT_LAYOUT.gap,
      ...(options?.gap ?? {}),
    },
    perRow: options?.perRow ?? DEFAULT_LAYOUT.perRow,
  };

  const element = createElementForNode(node);
  element.set("id", node.id);
  const isContainer =
    typeof node.container === "boolean"
      ? node.container
      : Boolean(options?.containerIds?.has(node.id));
  applyContainerMetadata(element, isContainer, resolveContainerPadding(node));
  const portLabelOffset = options?.portLabelOffset ?? 14;

  const labelOptions = options?.labelOptions;
  const compartmentMode = options?.compartmentMode ?? "mixed";
  const size = estimateNodeSize(
    node,
    resolveDefaultSize(node, layout.defaultSize),
    labelOptions,
    compartmentMode,
  );
  element.resize(size.width, size.height);

  const position = node.position ?? autoPosition(index, layout);
  element.position(position.x, position.y);

  const iconLabel = formatIconKindLabel(node, labelOptions);
  element.attr("kind/text", iconLabel);
  element.attr("label/text", formatNodeLabel(node, labelOptions));
  applyKindVariant(element, node);
  applyDefinitionUsageVariant(element, node);
  applyStatePresentation(element, node);
  applyMetaNotation(element, node);
  applyRequirementMetadata(element, node);
  applyExplicitStyle(element, node);
  applyNoteVariant(element, node);

  const textCompartments = filterTextCompartments(
    node.compartments,
    compartmentMode,
  );
  const graphicalCompartments = filterGraphicalCompartments(
    node.compartments,
    compartmentMode,
  );
  const detailsText = formatCompartmentText(
    stripRequirementIdLines(node, textCompartments),
  );
  if (detailsText) {
    element.attr("details/text", detailsText);
    element.attr("divider/display", "block");
  } else {
    element.attr("details/text", "");
    element.attr(
      "divider/display",
      graphicalCompartments.length > 0 ? "block" : "none",
    );
  }

  if (node.ports && node.ports.length > 0) {
    element.addPorts(
      node.ports.map((port) => {
        const group = port.side ?? port.group ?? port.direction ?? "inout";
        const args =
          port.offset !== undefined ? buildPortArgs(port) : undefined;
        const symbolAttrs = buildPortSymbolAttrs(port);
        const { labelText, ...portAttrs } = symbolAttrs;
        const labelAnchor = resolvePortLabelAnchor(resolvePortSide(port));
        return {
          id: port.id,
          group,
          args,
          label: {
            position: {
              name: "outsideOriented",
              args: { offset: portLabelOffset },
            },
          },
          attrs: {
            ...portAttrs,
            labelText: {
              ...labelAnchor,
              display: "none",
              ...(labelText ?? {}),
              text: formatPortLabel(port),
            },
          },
        };
      }),
    );
  }

  return element;
}

export function resolveNodePosition(
  node: VisSpecNode,
  position: VisSpecPoint | undefined,
  context: LayoutContext,
): VisSpecPoint | undefined {
  if (position) {
    return position;
  }
  if (context.allowSpecPositions) {
    return node.position;
  }
  return undefined;
}

export function resolveNodeForLayout(
  node: VisSpecNode,
  position: VisSpecPoint | undefined,
  context: LayoutContext,
): VisSpecNode {
  let resolved = node;
  if (!context.allowSpecPositions && node.position) {
    resolved = { ...resolved, position: undefined };
  }
  if (!context.allowSpecSizes && node.size) {
    resolved = { ...resolved, size: undefined };
  }
  if (position) {
    return { ...resolved, position };
  }
  return resolved;
}

function resolveDefaultSize(
  node: VisSpecNode,
  fallback: VisSpecSize,
): VisSpecSize {
  const kind = node.kind.toLowerCase();
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
  if (kind.includes("action")) {
    return { width: 180, height: 64 };
  }
  if (kind.includes("parameter")) {
    return {
      width: Math.min(fallback.width, 140),
      height: Math.min(fallback.height, 40),
    };
  }

  return fallback;
}

function createElementForNode(_node: VisSpecNode): dia.Element {
  const kind = _node.kind.toLowerCase();

  if (kind.includes("package")) {
    return new SysmlPackage();
  }
  if (kind.includes("usecase") || kind.includes("use case")) {
    return new SysmlUseCase();
  }
  if (kind.includes("actor")) {
    return new SysmlActor();
  }
  if (kind.includes("action")) {
    return new SysmlAction();
  }
  if (kind.includes("state")) {
    return new SysmlState();
  }
  if (kind.includes("requirement")) {
    return new SysmlRequirement();
  }
  if (kind.includes("constraint")) {
    return new SysmlConstraint();
  }
  if (kind.includes("parameter")) {
    return new SysmlParameter();
  }
  if (kind.includes("part") || kind.includes("block")) {
    return new SysmlBlock();
  }

  return new SysmlNode();
}

function applyNoteVariant(element: dia.Element, node: VisSpecNode): void {
  const kind = node.kind.toLowerCase();
  if (!kind.includes("note") && !kind.includes("comment")) {
    return;
  }

  element.attr("body/fill", "var(--sysml-grid-header)");
  element.attr("body/strokeDasharray", "4 3");
  element.attr("kind/text", "«note»");
}

function applyStatePresentation(element: dia.Element, node: VisSpecNode): void {
  const kind = node.kind.toLowerCase();
  if (!kind.includes("state")) {
    return;
  }

  if (!node.stereotype && !node.icon) {
    element.attr("kind/text", "");
  }
}

function applyMetaNotation(element: dia.Element, node: VisSpecNode): void {
  const metaText = buildMetaText(node);
  if (!metaText) {
    element.attr("meta/text", "");
    element.attr("meta/display", "none");
    return;
  }

  element.attr("meta/text", metaText);
  element.attr("meta/display", "block");
  element.attr("body/strokeDasharray", "6 3");
  element.attr("body/strokeWidth", 2);

  if (!node.style?.stroke) {
    element.attr("body/stroke", "var(--sysml-metaclass-stroke)");
  }
}

function applyRequirementMetadata(
  element: dia.Element,
  node: VisSpecNode,
): void {
  if (!node.kind.toLowerCase().includes("requirement")) {
    return;
  }

  const requirementId = extractRequirementId(node.compartments);
  if (!requirementId) {
    return;
  }

  const existingMeta = element.attr("meta/text");
  if (typeof existingMeta === "string" && existingMeta.trim().length > 0) {
    return;
  }

  element.attr("meta/text", requirementId);
  element.attr("meta/display", "block");
}

function applyDefinitionUsageVariant(
  element: dia.Element,
  node: VisSpecNode,
): void {
  const role = resolveDefinitionUsageRole(node.kind);
  if (!role || !shouldApplyDefinitionUsageVariant(node.kind)) {
    return;
  }

  const radius = role === "definition" ? 2 : 12;
  element.attr("body/rx", radius);
  element.attr("body/ry", radius);

  const tabRadius = Math.max(1, Math.round(radius / 2));
  if (element.attr("tab/rx") !== undefined) {
    element.attr("tab/rx", tabRadius);
  }
  if (element.attr("tab/ry") !== undefined) {
    element.attr("tab/ry", tabRadius);
  }
}

function extractRequirementId(
  compartments?: VisSpecCompartment[],
): string | null {
  if (!compartments) {
    return null;
  }

  for (const compartment of compartments) {
    if (compartment.kind !== "text" || !compartment.lines) {
      continue;
    }
    for (const line of compartment.lines) {
      const match = matchRequirementId(line);
      if (!match) {
        continue;
      }
      const label = match[1];
      const value = match[2].trim();
      if (!value) {
        continue;
      }
      return `${label}: ${value}`;
    }
  }

  return null;
}

function stripRequirementIdLines(
  node: VisSpecNode,
  compartments: VisSpecCompartment[] | undefined = node.compartments,
): VisSpecCompartment[] | undefined {
  if (!node.kind.toLowerCase().includes("requirement")) {
    return compartments;
  }

  if (!compartments || compartments.length === 0) {
    return compartments;
  }

  const filtered = compartments
    .map((compartment) => {
      if (compartment.kind !== "text" || !compartment.lines) {
        return compartment;
      }
      const lines = compartment.lines.filter(
        (line) => !matchRequirementId(line),
      );
      return { ...compartment, lines };
    })
    .filter((compartment) => {
      if (compartment.kind !== "text") {
        return true;
      }
      return (compartment.lines?.length ?? 0) > 0;
    });

  return filtered;
}

function matchRequirementId(line: string): RegExpMatchArray | null {
  const idPattern =
    /^\s*(id|identifier|req(?:uire(?:ment)?)?id?)\s*[:=]\s*(.+)\s*$/i;
  return line.match(idPattern);
}

function buildMetaText(node: VisSpecNode): string | null {
  const parts: string[] = [];

  if (node.metaclass) {
    parts.push(`«metaclass» ${node.metaclass}`);
  }

  if (node.metadata) {
    parts.push(`«metadata» ${node.metadata}`);
  }

  if (parts.length === 0) {
    return null;
  }

  return parts.join(" • ");
}

export function autoPosition(
  index: number,
  layout: LayoutOptions,
): VisSpecPoint {
  const row = Math.floor(index / layout.perRow);
  const col = index % layout.perRow;

  return {
    x: layout.padding.x + col * layout.gap.x,
    y: layout.padding.y + row * layout.gap.y,
  };
}

function formatCompartmentText(
  compartments?: VisSpecCompartment[],
): string | null {
  if (!compartments || compartments.length === 0) {
    return null;
  }

  const lines: string[] = [];
  compartments
    .filter((compartment) => compartment.kind === "text")
    .forEach((compartment) => {
      if (compartment.title) {
        lines.push(compartment.title);
      }
      if (compartment.lines && compartment.lines.length > 0) {
        compartment.lines.forEach((line) => {
          lines.push(`  ${line}`);
        });
      }
      lines.push("");
    });

  while (lines.length > 0 && lines[lines.length - 1] === "") {
    lines.pop();
  }

  return lines.length > 0 ? lines.join("\n") : null;
}

function formatPortLabel(port: VisSpecPort): string {
  return port.name;
}

function buildPortArgs(port: VisSpecPort): {
  x?: number | string;
  y?: number | string;
} {
  const offset = clampOffset(port.offset ?? 0.5);
  const percent = `${Math.round(offset * 1000) / 10}%`;
  const side = resolvePortSide(port);

  if (side === "left" || side === "right") {
    return { y: percent };
  }

  return { x: percent };
}

function resolvePortSide(
  port: VisSpecPort,
): "left" | "right" | "top" | "bottom" {
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

function resolvePortLabelAnchor(side: "left" | "right" | "top" | "bottom"): {
  textAnchor?: string;
  textVerticalAnchor?: string;
} {
  if (side === "left") {
    return { textAnchor: "end", textVerticalAnchor: "middle" };
  }
  if (side === "right") {
    return { textAnchor: "start", textVerticalAnchor: "middle" };
  }
  if (side === "top") {
    return { textAnchor: "middle", textVerticalAnchor: "bottom" };
  }
  return { textAnchor: "middle", textVerticalAnchor: "top" };
}

function clampOffset(value: number): number {
  if (Number.isNaN(value)) {
    return 0.5;
  }

  return Math.max(0, Math.min(1, value));
}

function applyExplicitStyle(element: dia.Element, node: VisSpecNode): void {
  if (node.style?.fill) {
    element.attr("body/fill", node.style.fill);
    element.attr("tab/fill", node.style.fill);
  }

  if (node.style?.stroke) {
    element.attr("body/stroke", node.style.stroke);
    element.attr("tab/stroke", node.style.stroke);
  }

  if (node.style?.text) {
    element.attr("label/fill", node.style.text);
    element.attr("details/fill", node.style.text);
  }
}

function applyKindVariant(element: dia.Element, node: VisSpecNode): void {
  const kind = node.kind.toLowerCase();
  const variant = kindVariantFor(kind);
  if (!variant) {
    return;
  }

  if (!node.style?.fill && variant.fill) {
    element.attr("body/fill", variant.fill);
    element.attr("tab/fill", variant.fill);
  }

  if (!node.style?.stroke && variant.stroke) {
    element.attr("body/stroke", variant.stroke);
    element.attr("tab/stroke", variant.stroke);
  }

  if (variant.text) {
    element.attr("kind/fill", variant.text);
  }
}

function kindVariantFor(kind: string): {
  fill?: string;
  stroke?: string;
  text?: string;
} | null {
  if (kind.includes("requirement")) {
    return {
      fill: "var(--sysml-req-fill)",
      stroke: "var(--sysml-req-stroke)",
    };
  }

  if (kind.includes("package")) {
    return {
      fill: "var(--sysml-package-fill)",
      stroke: "var(--sysml-package-stroke)",
    };
  }

  if (kind.includes("viewpoint") || kind.includes("view")) {
    return {
      fill: "var(--sysml-view-fill)",
      stroke: "var(--sysml-view-stroke)",
    };
  }

  if (kind.includes("constraint")) {
    return {
      fill: "var(--sysml-constraint-fill)",
      stroke: "var(--sysml-constraint-stroke)",
    };
  }

  if (kind.includes("state")) {
    return {
      fill: "var(--sysml-state-fill)",
      stroke: "var(--sysml-state-stroke)",
    };
  }

  if (kind.includes("action")) {
    return {
      fill: "var(--sysml-action-accent)",
      stroke: "var(--sysml-action)",
      text: "var(--sysml-muted)",
    };
  }

  if (kind.includes("connection")) {
    return {
      fill: "var(--sysml-connection-fill)",
      stroke: "var(--sysml-connection-stroke)",
    };
  }

  if (kind.includes("interface")) {
    return {
      fill: "var(--sysml-interface-fill)",
      stroke: "var(--sysml-interface-stroke)",
    };
  }

  return null;
}

export interface GraphicalCompartmentLayout {
  headerHeight?: number;
  gap?: number;
  padding?: number;
  lineHeight?: number;
  preservePositions?: boolean;
  sizeToChildren?: boolean;
  perRow?: number;
  gapX?: number;
  gapY?: number;
  allowSpecSizes?: boolean;
}

export function layoutGraphicalCompartments(
  spec: VisSpec,
  nodes: Map<string, dia.Element>,
  defaultSize: { width: number; height: number },
  compartmentMode: CompartmentMode = "mixed",
  layout?: GraphicalCompartmentLayout,
): void {
  if (compartmentMode === "textual") {
    return;
  }
  const nodeById = new Map(spec.nodes.map((node) => [node.id, node]));
  const headerHeight =
    layout?.headerHeight ?? GRAPHICAL_COMPARTMENT.headerHeight;
  const compartmentGap = layout?.gap ?? GRAPHICAL_COMPARTMENT.gap;
  const padding = layout?.padding ?? GRAPHICAL_COMPARTMENT.padding;
  const lineHeight = layout?.lineHeight ?? GRAPHICAL_COMPARTMENT.lineHeight;
  const preservePositions = layout?.preservePositions ?? false;
  const sizeToChildren =
    (layout?.sizeToChildren ?? false) && !preservePositions;
  const perRow = Math.max(1, layout?.perRow ?? DEFAULT_LAYOUT.perRow);
  const gapX = layout?.gapX ?? 16;
  const gapY = layout?.gapY ?? 16;
  const allowSpecSizes = layout?.allowSpecSizes ?? true;

  spec.nodes.forEach((parentSpec) => {
    if (!parentSpec.compartments || parentSpec.compartments.length === 0) {
      return;
    }

    const graphical = filterGraphicalCompartments(
      parentSpec.compartments,
      compartmentMode,
    );
    if (graphical.length === 0) {
      return;
    }

    const parentElement = nodes.get(parentSpec.id);
    if (!parentElement) {
      return;
    }

    const parentPosition = parentElement.position();
    const parentSize = parentElement.size();
    const textHeight =
      compartmentMode === "graphical"
        ? 0
        : estimateTextCompartmentHeight(
            parentSpec.compartments,
            lineHeight,
            compartmentGap,
          );

    const compartmentsWithNodes = graphical.filter(
      (compartment) => (compartment.nodeIds?.length ?? 0) > 0,
    );

    if (sizeToChildren) {
      const originX = parentPosition.x + padding;
      let cursorY = parentPosition.y + headerHeight + textHeight + padding;
      let placedCompartments = 0;
      let placedNodes = 0;

      compartmentsWithNodes.forEach((compartment) => {
        const nodeIds = (compartment.nodeIds ?? []).filter((id) =>
          nodes.has(id),
        );
        if (nodeIds.length === 0) {
          return;
        }

        if (
          preservePositions &&
          nodeIds.every((id) => nodeById.get(id)?.position)
        ) {
          return;
        }

        if (placedCompartments > 0) {
          cursorY += compartmentGap;
        }

        const { height, count } = layoutNodesInFlow(
          nodeIds,
          { x: originX, y: cursorY },
          nodes,
          nodeById,
          defaultSize,
          { perRow, gapX, gapY },
          allowSpecSizes,
        );

        if (count === 0) {
          if (placedCompartments > 0) {
            cursorY -= compartmentGap;
          }
          return;
        }

        cursorY += height;
        placedCompartments += 1;
        placedNodes += count;
      });

      if (placedCompartments === 0 || placedNodes === 0) {
        return;
      }

      const minRect = parentElement.get("sysmlMinRect") as
        | { width: number; height: number }
        | undefined;
      fitElementToChildren(parentElement, {
        padding: {
          left: padding,
          right: padding,
          top: headerHeight + textHeight + padding,
          bottom: padding,
        },
        minRect,
      });
      return;
    }

    const usableHeight =
      parentSize.height - headerHeight - textHeight - padding * 2;
    if (usableHeight <= 0) {
      return;
    }

    const compartmentHeight =
      (usableHeight - compartmentGap * (graphical.length - 1)) /
      graphical.length;

    graphical.forEach((compartment, index) => {
      const nodeIds = compartment.nodeIds ?? [];
      if (nodeIds.length === 0) {
        return;
      }

      if (
        preservePositions &&
        nodeIds.every((id) => nodeById.get(id)?.position)
      ) {
        return;
      }

      const area = {
        x: parentPosition.x + padding,
        y:
          parentPosition.y +
          headerHeight +
          textHeight +
          padding +
          index * (compartmentHeight + compartmentGap),
        width: parentSize.width - padding * 2,
        height: compartmentHeight,
      };

      layoutNodesInArea(
        nodeIds,
        area,
        nodes,
        nodeById,
        defaultSize,
        allowSpecSizes,
      );
    });
  });
}

export function makeLinkLabel(
  text: string,
  index = 0,
  position = 0.5,
  offsetX = 0,
  offsetY?: number,
): {
  position: { distance: number; offset: { x: number; y: number } };
  attrs: { text: { text: string } };
} {
  const yOffset = offsetY !== undefined ? offsetY : index % 2 === 0 ? -12 : 12;

  return {
    position: { distance: position, offset: { x: offsetX, y: yOffset } },
    attrs: { text: { text } },
  };
}

export interface ContainerDetectionOptions {
  kindHints?: string[];
}

export function collectContainerIds(
  spec: VisSpec,
  options?: ContainerDetectionOptions,
): Set<string> {
  const containers = new Set<string>();
  const hints = (options?.kindHints ?? []).map((hint) => hint.toLowerCase());

  spec.nodes.forEach((node) => {
    if (node.container) {
      containers.add(node.id);
    }

    if (
      node.compartments?.some(
        (compartment) =>
          compartment.kind === "graphical" &&
          (compartment.nodeIds?.length ?? 0) > 0,
      )
    ) {
      containers.add(node.id);
    }
  });

  spec.nodes.forEach((node) => {
    if (node.parentId) {
      containers.add(node.parentId);
    }
  });

  if (hints.length > 0) {
    spec.nodes.forEach((node) => {
      const kind = node.kind.toLowerCase();
      if (hints.some((hint) => kind.includes(hint))) {
        containers.add(node.id);
      }
    });
  }

  return containers;
}

type ContainerPadding = {
  left: number;
  right: number;
  top: number;
  bottom: number;
};

export function applyContainerMetadata(
  element: dia.Element,
  isContainer: boolean,
  padding?: ContainerPadding,
): void {
  if (isContainer) {
    const existingPadding = element.get("sysmlContainerPadding") as
      | ContainerPadding
      | undefined;
    const resolvedPadding = padding ?? existingPadding ?? CONTAINER_PADDING;
    element.set("sysmlContainer", true);
    element.set("sysmlContainerPadding", resolvedPadding);
    element.set("z", ZOrder.Container);
  } else {
    element.set("z", ZOrder.Node);
  }
}

export function applyLinkZOrder(link: dia.Link): void {
  link.set("z", ZOrder.Link);
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

function layoutNodesInArea(
  nodeIds: string[],
  area: { x: number; y: number; width: number; height: number },
  nodes: Map<string, dia.Element>,
  nodeById: Map<string, VisSpecNode>,
  defaultSize: { width: number; height: number },
  allowSpecSizes: boolean,
): void {
  const gapX = 16;
  const gapY = 16;
  const maxWidth = Math.max(40, area.width - gapX);
  const maxHeight = Math.max(40, area.height - gapY);

  let cursorX = area.x;
  let cursorY = area.y;
  let rowHeight = 0;

  nodeIds.forEach((id) => {
    const element = nodes.get(id);
    if (!element) {
      return;
    }

    const specNode = nodeById.get(id);
    const existingSize = element.size();
    const size = allowSpecSizes
      ? (specNode?.size ?? existingSize ?? defaultSize)
      : (existingSize ?? defaultSize);
    const width = Math.min(size.width, maxWidth);
    const height = Math.min(size.height, maxHeight);

    if (cursorX + width > area.x + area.width && cursorX > area.x) {
      cursorX = area.x;
      cursorY += rowHeight + gapY;
      rowHeight = 0;
    }

    element.resize(width, height, { sysmlAutoFit: true });
    element.position(cursorX, cursorY, { sysmlAutoFit: true });

    cursorX += width + gapX;
    rowHeight = Math.max(rowHeight, height);
  });
}

function layoutNodesInFlow(
  nodeIds: string[],
  origin: { x: number; y: number },
  nodes: Map<string, dia.Element>,
  nodeById: Map<string, VisSpecNode>,
  defaultSize: { width: number; height: number },
  options: { perRow: number; gapX: number; gapY: number },
  allowSpecSizes: boolean,
): { width: number; height: number; count: number } {
  const perRow = Math.max(1, options.perRow);
  let cursorX = origin.x;
  let cursorY = origin.y;
  let rowHeight = 0;
  let rowWidth = 0;
  let maxWidth = 0;
  let totalHeight = 0;
  let rows = 0;
  let column = 0;
  let count = 0;

  nodeIds.forEach((id) => {
    const element = nodes.get(id);
    if (!element) {
      return;
    }

    const specNode = nodeById.get(id);
    const existingSize = element.size();
    const size = allowSpecSizes
      ? (specNode?.size ?? existingSize ?? defaultSize)
      : (existingSize ?? defaultSize);

    element.resize(size.width, size.height, { sysmlAutoFit: true });
    element.position(cursorX, cursorY, { sysmlAutoFit: true });

    count += 1;
    rowWidth = Math.max(rowWidth, cursorX - origin.x + size.width);
    rowHeight = Math.max(rowHeight, size.height);
    column += 1;

    if (column >= perRow) {
      maxWidth = Math.max(maxWidth, rowWidth);
      totalHeight += rowHeight;
      rows += 1;
      column = 0;
      cursorX = origin.x;
      cursorY += rowHeight + options.gapY;
      rowHeight = 0;
      rowWidth = 0;
    } else {
      cursorX += size.width + options.gapX;
    }
  });

  if (column > 0) {
    maxWidth = Math.max(maxWidth, rowWidth);
    totalHeight += rowHeight;
    rows += 1;
  }

  if (rows > 1) {
    totalHeight += (rows - 1) * options.gapY;
  }

  return { width: maxWidth, height: totalHeight, count };
}

type PortSymbolAttrs = {
  portBody: Record<string, unknown>;
  portDecoration: Record<string, unknown>;
  labelText?: Record<string, unknown>;
};

function buildPortSymbolAttrs(port: VisSpecPort): PortSymbolAttrs {
  const symbol = resolvePortSymbol(port);
  const directionColor = resolvePortDirectionColor(port.direction);
  const typeFill = resolvePortFill(port, symbol);
  const typeStroke = resolvePortTypeStroke(symbol);
  const bodyCorner = symbol === "square" ? 2 : 6;
  const bodyStroke = symbol === "lollipop" ? typeStroke : directionColor;

  return {
    portBody: {
      rx: bodyCorner,
      ry: bodyCorner,
      fill: typeFill,
      stroke: bodyStroke ?? typeStroke,
      strokeWidth: directionColor ? 1.6 : 1.4,
    },
    portDecoration: {
      display: symbol === "lollipop" ? "block" : "none",
      stroke: directionColor ?? typeStroke,
      strokeWidth: directionColor ? 1.6 : 1.4,
    },
    labelText: directionColor
      ? {
          fill: directionColor,
          fontWeight: 600,
        }
      : undefined,
  };
}

function resolvePortSymbol(
  port: VisSpecPort,
): NonNullable<VisSpecPort["symbol"]> {
  if (port.symbol) {
    return port.symbol;
  }

  const hint = `${port.kind ?? ""} ${port.typedBy ?? ""}`.toLowerCase();
  if (hint.includes("interface")) {
    return "lollipop";
  }
  if (hint.includes("parameter") || hint.includes("param")) {
    return "circle";
  }

  return "square";
}

function resolvePortDirectionColor(
  direction?: VisSpecPort["direction"],
): string | undefined {
  if (direction === "in") {
    return "var(--sysml-port-in)";
  }
  if (direction === "out") {
    return "var(--sysml-port-out)";
  }
  if (direction === "inout") {
    return "var(--sysml-port-inout)";
  }
  return undefined;
}

function resolvePortTypeFill(
  symbol: NonNullable<VisSpecPort["symbol"]>,
): string {
  if (symbol === "lollipop") {
    return "var(--sysml-interface-fill)";
  }
  if (symbol === "circle") {
    return "var(--sysml-param-fill)";
  }

  return "var(--sysml-node-fill)";
}

function resolvePortFill(
  port: VisSpecPort,
  symbol: NonNullable<VisSpecPort["symbol"]>,
): string {
  const typeColor = resolvePortTypeColor(port);
  if (typeColor) {
    return typeColor;
  }
  return resolvePortTypeFill(symbol);
}

function resolvePortTypeColor(port: VisSpecPort): string | null {
  const key = (port.typedBy ?? port.kind ?? "").trim();
  if (!key) {
    return null;
  }
  const hue = hashToHue(key);
  return `hsl(${hue}, 38%, 78%)`;
}

function resolvePortTypeStroke(
  symbol: NonNullable<VisSpecPort["symbol"]>,
): string {
  if (symbol === "lollipop") {
    return "var(--sysml-interface-stroke)";
  }

  return "var(--sysml-node-stroke)";
}

function hashToHue(value: string): number {
  let hash = 0;
  for (let i = 0; i < value.length; i += 1) {
    hash = (hash * 31 + value.charCodeAt(i)) | 0;
  }
  return Math.abs(hash) % 360;
}

function resolveContainerPadding(node: VisSpecNode): ContainerPadding {
  const kind = node.kind.toLowerCase();
  if (kind.includes("partition") || kind.includes("swimlane")) {
    return { left: 48, right: 48, top: 96, bottom: 56 };
  }

  return CONTAINER_PADDING;
}
