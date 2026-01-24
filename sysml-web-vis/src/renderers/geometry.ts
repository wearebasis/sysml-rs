import { dia } from "@joint/core";
import { LayoutConfig, layoutSpec } from "../layout";
import { SysmlGeometryMarker, SysmlLink } from "../shapes";
import {
  VisSpec,
  VisSpecLink,
  VisSpecNode,
  VisSpecSpatialPoint,
  VisSpecSpatialSize,
} from "../vis-spec";
import { applyLinkZOrder, buildNode, makeLinkLabel } from "./common";
import { buildLinkLabels } from "./labels";
import { buildMarker } from "./markers";
import { resolveLabelingOptions } from "../notation";
import { resolveCompartmentMode } from "../compartments";

export function renderGeometryView(
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

  const origin = resolveOrigin(spec, layout.config.padding);
  const scale = spec.geometry?.scale ?? 1;

  if (spec.geometry?.origin) {
    const marker = new SysmlGeometryMarker();
    const markerSize = marker.size();
    marker.position(
      origin.x - markerSize.width / 2,
      origin.y - markerSize.height / 2,
    );
    marker.attr(
      "label/text",
      spec.geometry?.units ? `origin (${spec.geometry.units})` : "origin",
    );
    graph.addCell(marker);
  }

  spec.nodes.forEach((node, index) => {
    const position = resolveNodePosition(
      node,
      layout,
      origin,
      scale,
      layoutContext.allowSpecPositions,
    );
    const size = resolveNodeSize(node, layout, scale, layoutContext);
    const element = buildNode({ ...node, position, size }, index, nodeLayout);

    if (!node.style?.fill) {
      element.attr("body/fill", "var(--sysml-geometry-fill)");
    }
    if (!node.style?.stroke) {
      element.attr("body/stroke", "var(--sysml-geometry-stroke)");
    }

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
    // Keep geometry links straight to preserve explicit geometry points.

    const stroke = linkSpec.style?.stroke ?? "var(--sysml-geometry-link)";
    link.attr("line/stroke", stroke);
    const dashed =
      linkSpec.geometry?.style === "dashed" || linkSpec.lineStyle === "dashed";
    link.attr("line/strokeDasharray", dashed ? "6 4" : null);
    link.attr(
      "line/sourceMarker",
      buildMarker(linkSpec.markerStart ?? "none", stroke),
    );
    link.attr(
      "line/targetMarker",
      buildMarker(linkSpec.markerEnd ?? "triangle", stroke),
    );

    const vertices = resolveLinkVertices(linkSpec, origin, scale);
    if (vertices.length > 0) {
      link.vertices(vertices);
    }

    const explicitLabels = buildLinkLabels(linkSpec.labels);
    if (explicitLabels.length > 0) {
      link.labels(explicitLabels);
    } else {
      link.labels([makeLinkLabel(linkSpec.kind, 0, 0.5)]);
    }

    graph.addCell(link);
  });
}

function resolveOrigin(
  spec: VisSpec,
  fallback: { x: number; y: number },
): VisSpecSpatialPoint {
  return spec.geometry?.origin ?? { x: fallback.x, y: fallback.y };
}

function resolveNodePosition(
  node: VisSpecNode,
  layout: ReturnType<typeof layoutSpec>,
  origin: VisSpecSpatialPoint,
  scale: number,
  allowManualPosition: boolean,
): { x: number; y: number } {
  const spatial = node.geometry?.position;
  if (spatial) {
    return projectPoint(spatial, origin, scale);
  }

  if (allowManualPosition && node.position) {
    return node.position;
  }

  return (
    layout.positions.get(node.id) ?? {
      x: origin.x,
      y: origin.y,
    }
  );
}

function resolveNodeSize(
  node: VisSpecNode,
  layout: ReturnType<typeof layoutSpec>,
  scale: number,
  context: { allowSpecSizes: boolean },
): VisSpecSpatialSize {
  const spatial = node.geometry?.size;
  if (spatial) {
    return {
      width: spatial.width * scale,
      height: spatial.height * scale,
    };
  }

  if (context.allowSpecSizes && node.size) {
    return node.size;
  }
  return layout.config.defaultSize;
}

function resolveLinkVertices(
  link: VisSpecLink,
  origin: VisSpecSpatialPoint,
  scale: number,
): { x: number; y: number }[] {
  const points = link.geometry?.points;
  if (!points || points.length < 2) {
    return [];
  }

  const projected = points.map((point) => projectPoint(point, origin, scale));
  if (projected.length <= 2) {
    return [];
  }

  return projected.slice(1, projected.length - 1);
}

function projectPoint(
  point: VisSpecSpatialPoint,
  origin: VisSpecSpatialPoint,
  scale: number,
): { x: number; y: number } {
  return {
    x: origin.x + point.x * scale,
    y: origin.y + point.y * scale,
  };
}
