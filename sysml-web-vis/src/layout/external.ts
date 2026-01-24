import { dia } from "@joint/core";
import { FIT_TO_CONTENT_PADDING } from "../config/constants";
import type { ExternalLayoutStrategy, LayoutDirection } from "./index";

type ElkNode = {
  id?: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  children?: ElkNode[];
  edges?: ElkExtendedEdge[];
  layoutOptions?: Record<string, string>;
};
type ElkExtendedEdge = {
  id?: string;
  sources?: string[];
  targets?: string[];
  sections?: { bendPoints?: { x: number; y: number }[] }[];
};
type ElkGraph = ElkNode & { children: ElkNode[]; edges: ElkExtendedEdge[] };

const ELK_DIRECTION: Record<LayoutDirection, string> = {
  LR: "RIGHT",
  TB: "DOWN",
};

export async function applyExternalLayout(
  graph: dia.Graph,
  paper: dia.Paper,
  strategy: ExternalLayoutStrategy,
  direction: LayoutDirection,
): Promise<void> {
  if (strategy === "dagre") {
    const module = await safeImport("@joint/layout-directed-graph");
    if (!module) {
      return;
    }
    const { DirectedGraph } = module as {
      DirectedGraph: { layout: (cells: dia.Cell[], options: unknown) => void };
    };
    DirectedGraph.layout(graph.getCells(), {
      setVertices: true,
      setLabels: true,
      rankDir: direction,
      rankSep: 120,
      nodeSep: 80,
      edgeSep: 60,
    });
    paper.fitToContent({
      padding: FIT_TO_CONTENT_PADDING,
      allowNewOrigin: "any",
    });
    return;
  }

  if (strategy === "msagl") {
    const module = await safeImport("@joint/layout-msagl");
    if (!module) {
      return;
    }
    const { layout, LayerDirectionEnum, EdgeRoutingMode } = module as {
      layout: (graph: dia.Graph, options: unknown) => void;
      LayerDirectionEnum: { LR: number; TB: number };
      EdgeRoutingMode: { Rectilinear: number };
    };
    const layerDirection =
      direction === "LR" ? LayerDirectionEnum.LR : LayerDirectionEnum.TB;

    layout(graph, {
      layerDirection,
      edgeRoutingMode: EdgeRoutingMode.Rectilinear,
      layerSeparation: 140,
      nodeSeparation: 80,
      clusterPadding: 24,
      x: 20,
      y: 20,
      setVertices: true,
      setLabels: true,
    });
    paper.fitToContent({
      padding: FIT_TO_CONTENT_PADDING,
      allowNewOrigin: "any",
    });
    return;
  }

  if (strategy === "elk") {
    const module = await safeImport("elkjs/lib/elk-api.js");
    if (!module) {
      return;
    }
    const { default: ELK } = module as {
      default: new () => { layout: (graph: ElkGraph) => Promise<ElkGraph> };
    };
    const elk = new ELK();
    const elkGraph = buildElkGraph(graph, direction);
    const layouted = (await elk.layout(elkGraph)) as ElkGraph;
    applyElkLayout(graph, layouted);
    paper.fitToContent({
      padding: FIT_TO_CONTENT_PADDING,
      allowNewOrigin: "any",
    });
  }
}

async function safeImport(moduleId: string): Promise<unknown | null> {
  try {
    // @ts-expect-error Vite optional dependency
    return await import(/* @vite-ignore */ moduleId);
  } catch (error) {
    console.warn(
      `[sysml-web-vis] Optional dependency missing for ${moduleId}.`,
      error,
    );
    return null;
  }
}

function buildElkGraph(graph: dia.Graph, direction: LayoutDirection): ElkGraph {
  const elkNodes = new Map<string, ElkNode>();
  const roots: ElkNode[] = [];

  graph.getElements().forEach((element) => {
    const size = element.size();
    elkNodes.set(element.id.toString(), {
      id: element.id.toString(),
      width: size.width,
      height: size.height,
      children: [],
    });
  });

  graph.getElements().forEach((element) => {
    const elkNode = elkNodes.get(element.id.toString());
    if (!elkNode) {
      return;
    }

    const parent = element.getParentCell();
    if (parent && parent.isElement()) {
      const elkParent = elkNodes.get(parent.id.toString());
      if (elkParent) {
        (elkParent.children ??= []).push(elkNode);
        return;
      }
    }
    roots.push(elkNode);
  });

  const edges: ElkExtendedEdge[] = [];
  graph.getLinks().forEach((link) => {
    const source = link.getSourceElement();
    const target = link.getTargetElement();
    if (!source || !target) {
      return;
    }
    edges.push({
      id: link.id.toString(),
      sources: [source.id.toString()],
      targets: [target.id.toString()],
    });
  });

  return {
    id: "root",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.direction": ELK_DIRECTION[direction],
      "elk.layered.spacing.nodeNodeBetweenLayers": "120",
      "elk.spacing.nodeNode": "80",
      "elk.edgeRouting": "ORTHOGONAL",
    },
    children: roots,
    edges,
  };
}

function applyElkLayout(graph: dia.Graph, layouted: ElkGraph): void {
  graph.startBatch("elk-layout");

  const applyNode = (node: ElkNode) => {
    if (node.id) {
      const cell = graph.getCell(node.id);
      if (cell && cell.isElement()) {
        if (typeof node.width === "number" && typeof node.height === "number") {
          cell.resize(node.width, node.height);
        }
        if (typeof node.x === "number" && typeof node.y === "number") {
          cell.position(node.x, node.y);
        }
      }
    }
    if (node.children) {
      node.children.forEach(applyNode);
    }
  };

  layouted.children?.forEach(applyNode);

  layouted.edges?.forEach((edge) => {
    if (!edge.id) {
      return;
    }
    const cell = graph.getCell(edge.id);
    if (!cell || !cell.isLink()) {
      return;
    }
    const section = edge.sections?.[0];
    if (section?.bendPoints && section.bendPoints.length > 0) {
      cell.vertices(
        section.bendPoints.map((point) => ({ x: point.x, y: point.y })),
      );
      cell.set("router", null);
    }
  });

  graph.stopBatch("elk-layout");
}
