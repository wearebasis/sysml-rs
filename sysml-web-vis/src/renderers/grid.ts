import { dia } from "@joint/core";
import { LayoutConfig, layoutSpec } from "../layout";
import {
  SysmlGridCell,
  SysmlGridHeader,
  SysmlGridLine,
  SysmlLink,
} from "../shapes";
import { VisSpec, VisSpecGrid, VisSpecLink, VisSpecNode } from "../vis-spec";
import {
  applyLinkZOrder,
  buildNode,
  resolveNodeForLayout,
  ZOrder,
} from "./common";
import {
  formatKindLabel,
  formatNodeLabel,
  LabelingOptions,
  resolveLabelingOptions,
} from "../notation";
import { resolveCompartmentMode } from "../compartments";

export function renderGridView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  if (spec.grid) {
    renderGrid(
      graph,
      spec,
      spec.grid,
      resolveLabelingOptions(spec.viewMetadata),
    );
    return;
  }

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

  spec.nodes.forEach((node, index) => {
    const position = layout.positions.get(node.id);
    const element = buildNode(
      resolveNodeForLayout(node, position, layoutContext),
      index,
      nodeLayout,
    );
    graph.addCell(element);
    nodes.set(node.id, element);
  });

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
    link.attr(
      "line/stroke",
      linkSpec.style?.stroke ?? "var(--sysml-link-stroke)",
    );

    if (linkSpec.labels && linkSpec.labels.length > 0) {
      link.labels(
        linkSpec.labels.map((label) => ({
          position: label.position ?? 0.5,
          attrs: {
            text: {
              text: label.text,
            },
          },
        })),
      );
    }

    graph.addCell(link);
  });
}

function renderGrid(
  graph: dia.Graph,
  spec: VisSpec,
  grid: VisSpecGrid,
  labelOptions: LabelingOptions,
): void {
  const headerHeight = 36;
  const headerWidth = 200;
  const cellWidth = 200;
  const cellHeight = 70;
  const groupHeaderHeight = 24;
  const groupHeaderWidth = 140;
  const padding = { x: 60, y: 100 };
  const nodeById = new Map(spec.nodes.map((node) => [node.id, node]));
  const linkById = new Map(spec.links.map((link) => [link.id, link]));

  renderGridLines(graph, grid, padding, headerHeight, cellWidth, cellHeight);

  renderColumnGroups(graph, grid, padding, cellWidth, groupHeaderHeight);

  grid.columns.forEach((column, index) => {
    const header = new SysmlGridHeader();
    header.set("id", `col-${column.id}`);
    header.resize(cellWidth, headerHeight);
    header.position(padding.x + cellWidth * index, padding.y);
    header.set("z", ZOrder.Node);
    const headerText = formatAxisLabel(column, nodeById, labelOptions);
    header.attr("label/text", headerText);
    applyHeaderStyle(header, column.elementId, nodeById);
    graph.addCell(header);
  });

  renderRowGroups(
    graph,
    grid,
    padding,
    headerWidth,
    cellHeight,
    groupHeaderWidth,
    headerHeight,
  );

  grid.rows.forEach((row, rowIndex) => {
    const header = new SysmlGridHeader();
    header.set("id", `row-${row.id}`);
    header.resize(headerWidth, headerHeight);
    header.position(
      padding.x - headerWidth - 12,
      padding.y + headerHeight + cellHeight * rowIndex,
    );
    header.set("z", ZOrder.Node);
    const headerText = formatAxisLabel(row, nodeById, labelOptions);
    header.attr("label/text", headerText);
    applyHeaderStyle(header, row.elementId, nodeById);
    graph.addCell(header);
  });

  grid.cells.forEach((cell) => {
    const columnIndex = grid.columns.findIndex(
      (column) => column.id === cell.columnId,
    );
    const rowIndex = grid.rows.findIndex((row) => row.id === cell.rowId);

    if (columnIndex < 0 || rowIndex < 0) {
      return;
    }

    const cellElement = new SysmlGridCell();
    cellElement.set("id", `cell-${cell.rowId}-${cell.columnId}`);
    cellElement.resize(cellWidth, cellHeight);
    cellElement.position(
      padding.x + cellWidth * columnIndex,
      padding.y + headerHeight + cellHeight * rowIndex,
    );
    const links = resolveCellLinks(cell, linkById);
    const cellText = formatCellValues(cell, links);
    cellElement.attr("label/text", cellText);
    applyCellStyle(cellElement, cell, links);
    cellElement.set("z", ZOrder.Node);
    graph.addCell(cellElement);
  });
}

function formatAxisLabel(
  axis: { label: string; elementId?: string },
  nodeById: Map<string, VisSpecNode>,
  labelOptions: LabelingOptions,
): string {
  if (axis.elementId) {
    const node = nodeById.get(axis.elementId);
    if (node) {
      const kind = formatKindLabel(node, labelOptions);
      const name =
        node.name || node.label
          ? formatNodeLabel(node, labelOptions)
          : (axis.label ?? node.id);
      const formattedKind =
        kind && (kind.includes("<<") || kind.includes("«"))
          ? kind
          : kind
            ? `<<${kind}>>`
            : "";
      return formattedKind ? `${formattedKind}\n${name}` : name;
    }
  }

  return axis.label;
}

function formatCellValues(
  cell: { value?: string; values?: string[] },
  links: VisSpecLink[],
): string {
  const values: string[] = [];
  if (cell.value) {
    values.push(cell.value);
  }
  if (cell.values && cell.values.length > 0) {
    values.push(...cell.values);
  }
  if (values.length > 0) {
    return values.join("\n");
  }

  if (links.length === 0) {
    return "";
  }

  return links.map((link) => link.labels?.[0]?.text ?? link.kind).join("\n");
}

function applyHeaderStyle(
  header: dia.Element,
  elementId: string | undefined,
  nodeById: Map<string, VisSpecNode>,
): void {
  if (!elementId) {
    return;
  }

  const node = nodeById.get(elementId);
  if (!node) {
    return;
  }

  const kind = node.kind.toLowerCase();
  if (kind.includes("requirement")) {
    header.attr("body/fill", "var(--sysml-req-fill)");
    header.attr("body/stroke", "var(--sysml-req-stroke)");
  } else if (kind.includes("package")) {
    header.attr("body/fill", "var(--sysml-package-fill)");
    header.attr("body/stroke", "var(--sysml-package-stroke)");
  } else if (kind.includes("view")) {
    header.attr("body/fill", "var(--sysml-view-fill)");
    header.attr("body/stroke", "var(--sysml-view-stroke)");
  } else if (kind.includes("attribute")) {
    header.attr("body/fill", "var(--sysml-param-fill)");
    header.attr("body/stroke", "var(--sysml-node-stroke)");
  }
}

function applyCellStyle(
  cellElement: dia.Element,
  cell: { style?: { fill?: string; stroke?: string } },
  links: VisSpecLink[],
): void {
  if (cell.style?.fill) {
    cellElement.attr("body/fill", cell.style.fill);
  }
  if (cell.style?.stroke) {
    cellElement.attr("body/stroke", cell.style.stroke);
  }

  if (links.length === 0 || cell.style?.fill || cell.style?.stroke) {
    return;
  }

  const kind = resolveCellKind(links);
  if (!kind) {
    return;
  }
  const normalized = kind.toLowerCase();
  if (normalized.includes("satisfy")) {
    cellElement.attr("body/fill", "var(--sysml-req-fill)");
    cellElement.attr("body/stroke", "var(--sysml-req-stroke)");
  } else if (normalized.includes("verify")) {
    cellElement.attr("body/fill", "var(--sysml-constraint-fill)");
    cellElement.attr("body/stroke", "var(--sysml-constraint-stroke)");
  } else if (normalized.includes("allocate")) {
    cellElement.attr("body/fill", "var(--sysml-view-fill)");
    cellElement.attr("body/stroke", "var(--sysml-view-stroke)");
  } else if (normalized.includes("derive")) {
    cellElement.attr("body/fill", "var(--sysml-node-fill)");
    cellElement.attr("body/stroke", "var(--sysml-link-derive)");
  }
}

function resolveCellLinks(
  cell: { linkId?: string; linkIds?: string[] },
  linkById: Map<string, VisSpecLink>,
): VisSpecLink[] {
  const links: VisSpecLink[] = [];
  if (cell.linkId) {
    const link = linkById.get(cell.linkId);
    if (link) {
      links.push(link);
    }
  }
  if (cell.linkIds && cell.linkIds.length > 0) {
    cell.linkIds.forEach((id) => {
      const link = linkById.get(id);
      if (link) {
        links.push(link);
      }
    });
  }
  return links;
}

function resolveCellKind(links: VisSpecLink[]): string | null {
  if (links.length === 0) {
    return null;
  }
  const first = links[0].kind;
  if (links.every((link) => link.kind === first)) {
    return first;
  }
  return null;
}

function renderGridLines(
  graph: dia.Graph,
  grid: VisSpecGrid,
  padding: { x: number; y: number },
  headerHeight: number,
  cellWidth: number,
  cellHeight: number,
): void {
  const columns = grid.columns.length;
  const rows = grid.rows.length;
  const startX = padding.x;
  const startY = padding.y + headerHeight;
  const endX = padding.x + cellWidth * columns;
  const endY = padding.y + headerHeight + cellHeight * rows;

  for (let col = 0; col <= columns; col += 1) {
    const x = startX + col * cellWidth;
    const line = new SysmlGridLine();
    line.set("id", `grid-line-v-${col}`);
    line.attr("line/x1", x);
    line.attr("line/y1", startY);
    line.attr("line/x2", x);
    line.attr("line/y2", endY);
    line.set("z", ZOrder.Container);
    graph.addCell(line);
  }

  for (let row = 0; row <= rows; row += 1) {
    const y = startY + row * cellHeight;
    const line = new SysmlGridLine();
    line.set("id", `grid-line-h-${row}`);
    line.attr("line/x1", startX);
    line.attr("line/y1", y);
    line.attr("line/x2", endX);
    line.attr("line/y2", y);
    line.set("z", ZOrder.Container);
    graph.addCell(line);
  }
}

function renderColumnGroups(
  graph: dia.Graph,
  grid: VisSpecGrid,
  padding: { x: number; y: number },
  cellWidth: number,
  groupHeaderHeight: number,
): void {
  const groups = resolveAxisGroups(grid.columns);
  groups.forEach((group) => {
    const header = new SysmlGridHeader();
    header.set("id", `col-group-${group.label}-${group.start}`);
    header.resize(group.count * cellWidth, groupHeaderHeight);
    header.position(
      padding.x + group.start * cellWidth,
      padding.y - groupHeaderHeight - 8,
    );
    header.attr("label/text", group.label);
    applyGroupHeaderStyle(header);
    header.set("z", ZOrder.Node);
    graph.addCell(header);
  });
}

function renderRowGroups(
  graph: dia.Graph,
  grid: VisSpecGrid,
  padding: { x: number; y: number },
  headerWidth: number,
  cellHeight: number,
  groupHeaderWidth: number,
  headerHeight: number,
): void {
  const groups = resolveAxisGroups(grid.rows);
  groups.forEach((group) => {
    const header = new SysmlGridHeader();
    header.set("id", `row-group-${group.label}-${group.start}`);
    header.resize(groupHeaderWidth, group.count * cellHeight);
    header.position(
      padding.x - headerWidth - groupHeaderWidth - 20,
      padding.y + headerHeight + group.start * cellHeight,
    );
    header.attr("label/text", group.label);
    header.attr("label/textAnchor", "middle");
    header.attr("label/refX", "50%");
    header.attr("label/refY", "50%");
    applyGroupHeaderStyle(header);
    header.set("z", ZOrder.Node);
    graph.addCell(header);
  });
}

function resolveAxisGroups(
  axes: Array<{ group?: string }>,
): Array<{ label: string; start: number; count: number }> {
  const groups: Array<{ label: string; start: number; count: number }> = [];
  let current: { label: string; start: number; count: number } | null = null;

  axes.forEach((axis, index) => {
    if (!axis.group) {
      current = null;
      return;
    }

    if (!current || current.label !== axis.group) {
      current = { label: axis.group, start: index, count: 1 };
      groups.push(current);
      return;
    }

    current.count += 1;
  });

  return groups;
}

function applyGroupHeaderStyle(header: dia.Element): void {
  header.attr("body/fill", "var(--sysml-grid-group)");
  header.attr("body/stroke", "var(--sysml-grid-stroke)");
}
