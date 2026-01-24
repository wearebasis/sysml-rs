import { dia } from "@joint/core";
import { SysmlDiagramFrame, SysmlLegend } from "../shapes";
import { VisSpec, VisSpecLegend, VisSpecView } from "../vis-spec";
import { ZOrder } from "./common";

export interface DiagramFrameOptions {
  enabled?: boolean;
  padding?: number;
  label?: string;
}

export interface DiagramLegendOptions {
  enabled?: boolean;
  padding?: number;
}

export function applyDiagramFrame(
  graph: dia.Graph,
  paper: dia.Paper,
  spec: VisSpec,
  options: DiagramFrameOptions = {},
): void {
  if (!options.enabled) {
    return;
  }

  const existing = graph
    .getElements()
    .find((element) => element.get("type") === "sysml.DiagramFrame");
  if (existing) {
    existing.remove();
  }

  const bbox = paper.getContentBBox({ useModelGeometry: true });
  if (bbox.width <= 0 || bbox.height <= 0) {
    return;
  }

  const padding = options.padding ?? 24;
  const frame = new SysmlDiagramFrame();
  frame.position(bbox.x - padding, bbox.y - padding);
  frame.resize(bbox.width + padding * 2, bbox.height + padding * 2);
  frame.attr("label/text", buildFrameLabel(spec, options));
  frame.set("z", ZOrder.Container);
  graph.addCell(frame);
}

export function applyDiagramLegend(
  graph: dia.Graph,
  paper: dia.Paper,
  spec: VisSpec,
  options: DiagramLegendOptions = {},
): void {
  const legend = spec.viewMetadata?.legend;
  if (!legend || options.enabled === false) {
    return;
  }

  const existing = graph
    .getElements()
    .find((element) => element.get("type") === "sysml.Legend");
  if (existing) {
    existing.remove();
  }

  const bbox = paper.getContentBBox({ useModelGeometry: true });
  if (bbox.width <= 0 || bbox.height <= 0) {
    return;
  }

  const { width, height } = measureLegend(legend);
  const padding = options.padding ?? 16;
  const position = resolveLegendPosition(
    bbox,
    { width, height },
    legend.position ?? "bottom-right",
    padding,
  );

  const legendElement = new SysmlLegend();
  legendElement.position(position.x, position.y);
  legendElement.resize(width, height);
  legendElement.attr("title/text", legend.title ?? "");
  legendElement.attr("items/text", legend.items.join("\n"));

  if (!legend.title) {
    legendElement.attr("title/display", "none");
    legendElement.attr("items/refY", 18);
  } else {
    legendElement.attr("title/display", "block");
    legendElement.attr("items/refY", 36);
  }

  legendElement.set("z", ZOrder.Node);
  graph.addCell(legendElement);
}

function buildFrameLabel(spec: VisSpec, options: DiagramFrameOptions): string {
  if (options.label !== undefined) {
    return options.label;
  }

  const viewName = formatViewName(spec.view);
  const subject = spec.viewMetadata?.subject ?? spec.viewMetadata?.title;
  if (subject) {
    return `${viewName} - ${subject}`;
  }
  return viewName;
}

function formatViewName(view: VisSpecView): string {
  return view
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/\s+/g, " ")
    .trim();
}

function measureLegend(legend: VisSpecLegend): {
  width: number;
  height: number;
} {
  const title = legend.title ?? "";
  const items = legend.items ?? [];
  const lines = title ? [title, ...items] : items;
  const maxLength = Math.max(0, ...lines.map((line) => line.length));

  const approxCharWidth = 6;
  const padding = 32;
  const width = Math.max(
    180,
    Math.min(320, maxLength * approxCharWidth + padding),
  );

  const titleHeight = title ? 22 : 0;
  const lineHeight = 16;
  const height = Math.max(52, titleHeight + items.length * lineHeight + 16);

  return { width, height };
}

function resolveLegendPosition(
  bbox: { x: number; y: number; width: number; height: number },
  size: { width: number; height: number },
  position: NonNullable<VisSpecLegend["position"]>,
  padding: number,
): { x: number; y: number } {
  switch (position) {
    case "top-left":
      return { x: bbox.x + padding, y: bbox.y + padding };
    case "top-right":
      return {
        x: bbox.x + bbox.width - size.width - padding,
        y: bbox.y + padding,
      };
    case "bottom-left":
      return {
        x: bbox.x + padding,
        y: bbox.y + bbox.height - size.height - padding,
      };
    case "bottom-right":
    default:
      return {
        x: bbox.x + bbox.width - size.width - padding,
        y: bbox.y + bbox.height - size.height - padding,
      };
  }
}
