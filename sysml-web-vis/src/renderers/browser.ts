import { dia } from "@joint/core";
import { LayoutConfig, layoutSpec } from "../layout";
import { SysmlBrowserNode, SysmlLink } from "../shapes";
import { VisSpec, VisSpecLink, VisSpecNode } from "../vis-spec";
import { applyLinkZOrder, resolveNodeForLayout } from "./common";
import {
  formatKindLabel,
  formatNodeLabel,
  LabelingOptions,
  resolveLabelingOptions,
} from "../notation";
import { resolveCompartmentMode } from "../compartments";

export function renderBrowserView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const layout = layoutSpec(spec, layoutConfig);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
  const nodeLayout = {
    defaultSize: layout.config.defaultSize,
    padding: layout.config.padding,
    gap: { x: layout.config.layerGap, y: layout.config.nodeGap },
    perRow: layout.config.perRow,
    labelOptions,
    compartmentMode,
  };
  const layoutContext = layout.context;

  const metadata = spec.viewMetadata;
  const filterKinds = metadata?.filterKinds?.map((kind) => kind.toLowerCase());
  const sortBy = metadata?.sortBy ?? "name";
  const sortOrder = metadata?.sortOrder ?? "asc";

  const parentByChild = parentMapFromLinks(spec.links);
  const visibleNodes = spec.nodes.filter((node) =>
    matchesFilter(node, filterKinds),
  );
  const resolveBrowserSize = (node: VisSpecNode) =>
    layoutContext.allowSpecSizes && node.size
      ? node.size
      : layout.config.defaultSize;
  const maxHeight = Math.max(
    layout.config.defaultSize.height,
    ...visibleNodes.map((node) => resolveBrowserSize(node).height),
  );
  const nodeById = new Map(visibleNodes.map((node) => [node.id, node]));
  const childrenByParent = new Map<string, VisSpecNode[]>();
  const roots: VisSpecNode[] = [];

  visibleNodes.forEach((node) => {
    const parentId = node.parentId ?? parentByChild.get(node.id);
    if (!parentId || !nodeById.has(parentId)) {
      roots.push(node);
      return;
    }

    const siblings = childrenByParent.get(parentId) ?? [];
    siblings.push(node);
    childrenByParent.set(parentId, siblings);
  });

  const comparator = buildComparator(sortBy, sortOrder);
  roots.sort(comparator);
  childrenByParent.forEach((children) => children.sort(comparator));

  const indent = 40;
  const rowGap = Math.max(24, layout.config.nodeGap / 4);
  const rowStep = maxHeight + rowGap;
  let rowIndex = 0;

  const renderNode = (node: VisSpecNode, depth: number) => {
    const children = childrenByParent.get(node.id) ?? [];
    const hasChildren = children.length > 0;
    const labelPrefix = hasChildren ? (node.collapsed ? "+ " : "- ") : "";
    const label = `${labelPrefix}${node.name ?? node.label ?? node.id}`;
    const parentId = node.parentId ?? parentByChild.get(node.id);

    const resolvedNode = resolveNodeForLayout(
      { ...node, name: undefined, label },
      undefined,
      layoutContext,
    );
    const element = buildBrowserNode(resolvedNode, rowIndex, nodeLayout);
    const size = resolveBrowserSize(node);
    const x = layout.config.padding.x + depth * indent;
    const y = layout.config.padding.y + rowIndex * rowStep;
    element.resize(size.width, size.height);
    element.position(x, y);

    graph.addCell(element);
    nodes.set(node.id, element);
    rowIndex += 1;

    if (parentId && nodeById.has(parentId)) {
      const parentElement = nodes.get(parentId);
      if (parentElement) {
        const link = buildTreeLink(parentElement, element);
        graph.addCell(link);
      }
    }

    if (!node.collapsed) {
      children.forEach((child) => renderNode(child, depth + 1));
    }
  };

  roots.forEach((root) => renderNode(root, 0));
}

function buildTreeLink(parent: dia.Element, child: dia.Element): dia.Link {
  const link = new SysmlLink();
  link.source({ id: parent.id });
  link.target({ id: child.id });
  link.router({
    name: "manhattan",
    args: {
      padding: 8,
      step: 8,
      maxAllowedDirectionChange: 90,
      startDirections: ["right"],
      endDirections: ["left"],
    },
  });
  link.connector("rounded");
  link.attr("line/stroke", "var(--sysml-grid-stroke)");
  link.attr("line/strokeDasharray", "2 2");
  link.attr("line/strokeWidth", 1);
  link.attr("line/targetMarker", { type: "path", d: "M 0 0" });
  applyLinkZOrder(link);
  return link;
}

function matchesFilter(node: VisSpecNode, filterKinds?: string[]): boolean {
  if (!filterKinds || filterKinds.length === 0) {
    return true;
  }

  const kind = node.kind.toLowerCase();
  return filterKinds.some((filter) => kind.includes(filter));
}

function parentMapFromLinks(links: VisSpecLink[]): Map<string, string> {
  const parentByChild = new Map<string, string>();
  links.forEach((link) => {
    const normalized = link.kind.toLowerCase();
    if (
      normalized.includes("contain") ||
      normalized.includes("own") ||
      normalized.includes("member")
    ) {
      parentByChild.set(link.target.nodeId, link.source.nodeId);
    }
  });
  return parentByChild;
}

function buildComparator(
  sortBy: "name" | "kind",
  sortOrder: "asc" | "desc",
): (left: VisSpecNode, right: VisSpecNode) => number {
  const direction = sortOrder === "desc" ? -1 : 1;
  return (left, right) => {
    const leftValue =
      sortBy === "kind"
        ? left.kind.toLowerCase()
        : (left.name ?? left.label ?? left.id).toLowerCase();
    const rightValue =
      sortBy === "kind"
        ? right.kind.toLowerCase()
        : (right.name ?? right.label ?? right.id).toLowerCase();
    if (leftValue < rightValue) {
      return -1 * direction;
    }
    if (leftValue > rightValue) {
      return 1 * direction;
    }
    return 0;
  };
}

function buildBrowserNode(
  node: VisSpecNode,
  index: number,
  layout: {
    defaultSize: { width: number; height: number };
    padding: { x: number; y: number };
    gap: { x: number; y: number };
    perRow: number;
    labelOptions?: LabelingOptions;
  },
): dia.Element {
  const element = new SysmlBrowserNode();
  element.set("id", node.id);

  const size = node.size ?? layout.defaultSize;
  element.resize(size.width, size.height);

  const position = node.position ?? {
    x: layout.padding.x + (index % layout.perRow) * layout.gap.x,
    y: layout.padding.y + Math.floor(index / layout.perRow) * layout.gap.y,
  };
  element.position(position.x, position.y);

  element.attr("kind/text", formatKindLabel(node, layout.labelOptions));
  element.attr("label/text", formatNodeLabel(node, layout.labelOptions));

  const icon = resolveBrowserIcon(node.kind);
  if (icon) {
    element.attr("icon/display", "block");
    element.attr("icon/d", icon.d);
    element.attr("icon/fill", icon.fill ?? "none");
    element.attr("icon/stroke", icon.stroke ?? "var(--sysml-browser-icon)");
    element.attr("icon/strokeWidth", icon.strokeWidth ?? 1.4);
  }

  if (node.kind.toLowerCase().includes("package")) {
    element.attr("body/fill", "var(--sysml-browser)");
  }

  return element;
}

function resolveBrowserIcon(
  kind: string,
): { d: string; fill?: string; stroke?: string; strokeWidth?: number } | null {
  const normalized = kind.toLowerCase();
  if (normalized.includes("package")) {
    return { d: "M 0 3 L 8 3 L 10 6 L 20 6 L 20 16 L 0 16 Z" };
  }
  if (normalized.includes("requirement")) {
    return { d: "M 0 0 L 16 0 L 20 4 L 20 16 L 0 16 Z" };
  }
  if (normalized.includes("view")) {
    return { d: "M 2 2 L 18 2 L 18 14 L 2 14 Z" };
  }
  if (normalized.includes("constraint")) {
    return { d: "M 2 0 L 18 0 L 20 8 L 18 16 L 2 16 L 0 8 Z" };
  }
  if (normalized.includes("part") || normalized.includes("block")) {
    return { d: "M 0 0 L 18 0 L 18 16 L 0 16 Z" };
  }
  if (normalized.includes("action")) {
    return {
      d: "M 2 8 L 10 2 L 18 8 L 10 14 Z",
      fill: "var(--sysml-action-accent)",
      stroke: "var(--sysml-action)",
    };
  }
  if (normalized.includes("state")) {
    return {
      d: "M 2 2 L 18 2 L 18 14 L 2 14 Z",
      stroke: "var(--sysml-state-stroke)",
    };
  }
  if (normalized.includes("actor")) {
    return {
      d: "M 10 3 A 3 3 0 1 1 9.999 3 M 10 6 L 10 13 M 6 9 L 14 9 M 6 15 L 10 13 L 14 15",
      strokeWidth: 1.2,
    };
  }

  return null;
}
