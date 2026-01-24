import { dia } from "@joint/core";
import { LayoutConfig, layoutSpec } from "../layout";
import {
  SysmlDecisionNode,
  SysmlFinalNode,
  SysmlHistoryNode,
  SysmlLink,
  SysmlStartNode,
} from "../shapes";
import { VisSpec, VisSpecNode } from "../vis-spec";
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
import { buildLinkLabels, formatTransitionLabel } from "./labels";
import { applyParallelLinkOffset, buildParallelLinkIndex } from "./links";
import { ControlNodeDefinition, createControlNode } from "./positioning";
import { buildMarker } from "./markers";
import { resolveLabelingOptions } from "../notation";
import { resolveCompartmentMode } from "../compartments";

const STATE_CONTROL_NODES: ControlNodeDefinition[] = [
  {
    matches: (kind) => kind.includes("initial") || kind.includes("start"),
    create: () => new SysmlStartNode(),
  },
  {
    matches: (kind) => kind.includes("final") || kind.includes("terminate"),
    create: () => new SysmlFinalNode(),
  },
  {
    matches: (kind) => kind.includes("choice") || kind.includes("junction"),
    create: () => new SysmlDecisionNode(),
  },
  {
    matches: (kind) => kind.includes("history"),
    create: () => new SysmlHistoryNode(),
    afterCreate: (element, node) => {
      element.attr("label/text", resolveHistoryLabel(node));
    },
  },
];

export function renderStateTransitionView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  const nodes = new Map<string, dia.Element>();
  const layout = layoutSpec(spec, layoutConfig);
  const labelOptions = resolveLabelingOptions(spec.viewMetadata);
  const compartmentMode = resolveCompartmentMode(spec.viewMetadata);
  const containerIds = collectContainerIds(spec, { kindHints: ["region"] });
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
    const controlElement = createControlNode(
      resolvedNode,
      STATE_CONTROL_NODES,
      resolvedPosition,
      layout.config.defaultSize,
    );
    const element =
      controlElement ?? buildNode(resolvedNode, index, nodeLayout);
    if (controlElement) {
      applyStateControlStyling(controlElement, node);
    }
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

  layoutGraphicalCompartments(
    spec,
    nodes,
    layout.config.defaultSize,
    compartmentMode,
    {
      headerHeight: 64,
      gap: 14,
      padding: 18,
      preservePositions: layoutContext.allowSpecPositions,
      sizeToChildren: !layoutContext.allowSpecPositions,
      perRow: layout.config.perRow,
      allowSpecSizes: layoutContext.allowSpecSizes,
    },
  );

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

    const stroke = linkSpec.style?.stroke ?? "var(--sysml-state-stroke)";
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
      buildMarker(linkSpec.markerEnd ?? "triangle", stroke),
    );

    if (linkSpec.transition) {
      link.labels([
        makeLinkLabel(
          formatTransitionLabel(linkSpec.transition),
          0,
          0.5,
          0,
          -12,
        ),
      ]);
    } else {
      const explicitLabels = buildLinkLabels(linkSpec.labels, {
        offsetY: -10,
      });
      if (explicitLabels.length > 0) {
        link.labels(explicitLabels);
      }
    }

    graph.addCell(link);
  });
}

function applyStateControlStyling(
  element: dia.Element,
  node: VisSpecNode,
): void {
  const kind = node.kind.toLowerCase();
  const stroke = "var(--sysml-state-stroke)";
  const fill = "var(--sysml-state-fill)";

  if (kind.includes("initial") || kind.includes("start")) {
    element.attr("body/fill", stroke);
    element.attr("body/stroke", stroke);
    return;
  }

  if (kind.includes("final") || kind.includes("terminate")) {
    element.attr("outer/stroke", stroke);
    element.attr("inner/fill", stroke);
    element.attr("inner/stroke", stroke);
    return;
  }

  if (kind.includes("choice") || kind.includes("junction")) {
    element.attr("body/fill", fill);
    element.attr("body/stroke", stroke);
    return;
  }

  if (kind.includes("history")) {
    element.attr("body/fill", fill);
    element.attr("body/stroke", stroke);
    element.attr("label/fill", stroke);
  }
}

function resolveHistoryLabel(node: VisSpecNode): string {
  const kind = node.kind.toLowerCase();
  if (kind.includes("deep") || kind.includes("history*")) {
    return "H*";
  }
  if (node.name && node.name.includes("*")) {
    return "H*";
  }
  return "H";
}
