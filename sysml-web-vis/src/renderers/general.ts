import { dia } from "@joint/core";
import { LayoutConfig, layoutSpec } from "../layout";
import { SysmlLink } from "../shapes";
import { VisSpec, VisSpecLink } from "../vis-spec";
import {
  applyLinkZOrder,
  buildNode,
  makeLinkLabel,
  resolveNodeForLayout,
} from "./common";
import { buildLinkLabels } from "./labels";
import { applyParallelLinkOffset, buildParallelLinkIndex } from "./links";
import { buildMarker } from "./markers";
import { resolveLabelingOptions } from "../notation";
import { resolveCompartmentMode } from "../compartments";

export function renderGeneralView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const parallelIndex = buildParallelLinkIndex(spec.links);

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
    link.router({
      name: "manhattan",
      args: resolveGeneralRouterArgs(layout.config.direction),
    });
    link.connector("rounded");

    applyGeneralLinkStyle(link, linkSpec);
    applyParallelLinkOffset(
      link,
      source,
      target,
      parallelIndex.get(linkSpec.id),
    );

    const distance = measureLinkDistance(source, target);
    const offsetX = index % 2 === 0 ? 0 : 12;

    const explicitLabels = buildLinkLabels(linkSpec.labels, { offsetX });
    if (explicitLabels.length > 0) {
      link.labels(explicitLabels);
    } else if (shouldAutoLabelGeneral(linkSpec.kind, distance)) {
      link.labels([makeLinkLabel(linkSpec.kind, 0, 0.5, offsetX, -10)]);
    }

    graph.addCell(link);
  });
}

type MarkerSpec = {
  kind: "none" | "triangle" | "diamond" | "open";
  fill?: string;
};

function applyGeneralLinkStyle(link: dia.Link, linkSpec: VisSpecLink): void {
  const normalized = linkSpec.kind.toLowerCase();
  const style = resolveGeneralLinkStyle(normalized);

  const stroke = linkSpec.style?.stroke ?? style.stroke;
  link.attr("line/stroke", stroke);
  const dashed =
    linkSpec.lineStyle === "dashed"
      ? "6 4"
      : linkSpec.lineStyle === "solid"
        ? null
        : (style.dash ?? null);
  link.attr("line/strokeDasharray", dashed);

  const sourceMarker = resolveMarkerSpec(
    linkSpec.markerStart,
    style.markerStart,
  );
  const targetMarker = resolveMarkerSpec(linkSpec.markerEnd, style.markerEnd);

  link.attr(
    "line/sourceMarker",
    buildMarker(sourceMarker.kind, stroke, markerStyle(sourceMarker)),
  );
  link.attr(
    "line/targetMarker",
    buildMarker(targetMarker.kind, stroke, markerStyle(targetMarker)),
  );
}

function resolveMarkerSpec(
  override: MarkerSpec["kind"] | undefined,
  fallback: MarkerSpec | undefined,
): MarkerSpec {
  if (override) {
    return { kind: override };
  }
  return fallback ?? { kind: "none" };
}

function markerStyle(spec: MarkerSpec): { fill?: string } | undefined {
  if (!spec.fill) {
    return undefined;
  }
  return { fill: spec.fill };
}

function resolveGeneralLinkStyle(kind: string): {
  stroke: string;
  dash?: string;
  markerStart?: MarkerSpec;
  markerEnd?: MarkerSpec;
} {
  if (kind.includes("composition") || kind.includes("composite")) {
    return {
      stroke: "var(--sysml-link-stroke)",
      markerStart: { kind: "diamond" },
    };
  }

  if (kind.includes("aggregation") || kind.includes("aggregate")) {
    return {
      stroke: "var(--sysml-link-stroke)",
      markerStart: { kind: "diamond", fill: "none" },
    };
  }

  if (
    kind.includes("contain") ||
    kind.includes("own") ||
    kind.includes("member")
  ) {
    return {
      stroke: "var(--sysml-link-ownership)",
      dash: "6 4",
    };
  }

  if (
    kind.includes("import") ||
    kind.includes("merge") ||
    kind.includes("access")
  ) {
    return {
      stroke: "var(--sysml-link-ownership)",
      dash: "2 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("typeof") || kind.includes("type of")) {
    return {
      stroke: "var(--sysml-link-typing)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("instanceof") || kind.includes("instance of")) {
    return {
      stroke: "var(--sysml-link-typing)",
      dash: "2 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("type") || kind.includes("typing")) {
    return {
      stroke: "var(--sysml-link-typing)",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("specialization") || kind.includes("generalization")) {
    return {
      stroke: "var(--sysml-link-specialization)",
      markerEnd: { kind: "triangle", fill: "none" },
    };
  }

  if (kind.includes("satisfy")) {
    return {
      stroke: "var(--sysml-link-req)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("verify")) {
    return {
      stroke: "var(--sysml-link-verify)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("derive")) {
    return {
      stroke: "var(--sysml-link-derive)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("allocate")) {
    return {
      stroke: "var(--sysml-link-allocate)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (
    kind.includes("refine") ||
    kind.includes("trace") ||
    kind.includes("depend")
  ) {
    return {
      stroke: "var(--sysml-link-stroke)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (kind.includes("include") || kind.includes("extend")) {
    return {
      stroke: "var(--sysml-link-stroke)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  if (
    kind.includes("view") ||
    kind.includes("viewpoint") ||
    kind.includes("render") ||
    kind.includes("concern") ||
    kind.includes("stakeholder")
  ) {
    return {
      stroke: "var(--sysml-link-view)",
      dash: "4 4",
      markerEnd: { kind: "open" },
    };
  }

  return {
    stroke: "var(--sysml-link-stroke)",
  };
}

function shouldAutoLabelGeneral(kind: string, distance: number): boolean {
  const normalized = kind.toLowerCase();
  const suppressed = [
    "contain",
    "own",
    "member",
    "import",
    "merge",
    "access",
    "type",
    "instance",
    "specialization",
    "generalization",
    "association",
    "satisfy",
    "verify",
    "derive",
    "allocate",
    "refine",
    "trace",
    "depend",
    "include",
    "extend",
    "view",
    "viewpoint",
    "concern",
    "stakeholder",
  ];

  if (suppressed.some((term) => normalized.includes(term))) {
    return false;
  }

  return distance >= 160;
}

function resolveGeneralRouterArgs(
  direction: "LR" | "TB",
): Record<string, unknown> {
  if (direction === "LR") {
    return {
      padding: 20,
      step: 14,
      maxAllowedDirectionChange: 90,
      startDirections: ["right", "left"],
      endDirections: ["left", "right"],
    };
  }

  return {
    padding: 20,
    step: 14,
    maxAllowedDirectionChange: 90,
    startDirections: ["bottom", "left", "right"],
    endDirections: ["top", "left", "right"],
  };
}

function measureLinkDistance(source: dia.Element, target: dia.Element): number {
  const sourceBox = source.getBBox();
  const targetBox = target.getBBox();
  const dx =
    sourceBox.x + sourceBox.width / 2 - (targetBox.x + targetBox.width / 2);
  const dy =
    sourceBox.y + sourceBox.height / 2 - (targetBox.y + targetBox.height / 2);
  return Math.hypot(dx, dy);
}
