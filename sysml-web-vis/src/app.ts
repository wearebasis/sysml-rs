import { dia } from "@joint/core";
import { fitPaperToViewport, renderDiagram } from "./bootstrap";
import {
  buildExportName,
  downloadPng,
  downloadSvg,
  serializeSvgDocument,
} from "./export";
import {
  buildInteractionPaperOptions,
  enableInteractionConstraints,
  enableZoomAndPan,
  enablePortHoverLabels,
} from "./interaction";
import { LayoutStrategy } from "./layout";
import { buildPaperDefaults } from "./config/paper";
import { cellNamespace } from "./shapes";
import type { VisSpecView } from "./vis-spec";
import { initializeControls, RouterStrategy, updateDiagramHeader } from "./ui";
import { resolveFixtureSelection, viewOptions } from "./fixtures";

declare global {
  interface Window {
    __SYSML_EXPORT__?: {
      svg: () => string;
    };
  }
}

const LAYOUT_OPTIONS_BY_VIEW: Record<VisSpecView, LayoutStrategy[]> = {
  GeneralView: ["auto", "manual", "grid", "layered", "dagre", "msagl", "elk"],
  InterconnectionView: [
    "auto",
    "interconnection",
    "manual",
    "grid",
    "layered",
    "dagre",
    "msagl",
    "elk",
  ],
  ActionFlowView: ["auto", "manual", "layered", "dagre", "msagl", "elk"],
  StateTransitionView: ["auto", "manual", "layered", "dagre", "msagl", "elk"],
  SequenceView: ["auto", "manual"],
  BrowserView: ["auto"],
  GridView: ["auto"],
  GeometryView: ["manual"],
};

const ROUTER_OPTIONS_BY_VIEW: Record<VisSpecView, RouterStrategy[]> = {
  GeneralView: ["default", "avoid"],
  InterconnectionView: ["default", "avoid"],
  ActionFlowView: ["default", "avoid"],
  StateTransitionView: ["default", "avoid"],
  SequenceView: ["default"],
  BrowserView: ["default"],
  GridView: ["default"],
  GeometryView: ["default"],
};

function resolveLayoutOptions(view: VisSpecView): LayoutStrategy[] {
  return LAYOUT_OPTIONS_BY_VIEW[view] ?? ["auto"];
}

function resolveRouterOptions(view: VisSpecView): RouterStrategy[] {
  return ROUTER_OPTIONS_BY_VIEW[view] ?? ["default"];
}

export function startApp(): void {
  const app = document.getElementById("app");
  if (!app) {
    throw new Error("Missing #app container");
  }

  const graph = new dia.Graph({}, { cellNamespace });

  const paper = new dia.Paper({
    el: app,
    model: graph,
    width: app.clientWidth,
    height: app.clientHeight,
    gridSize: 16,
    drawGrid: {
      name: "mesh",
      args: { color: "var(--sysml-grid)", thickness: 1 },
    },
    background: { color: "var(--sysml-bg)" },
    interactive: { labelMove: true },
    snapLabels: true,
    labelsLayer: true,
    sorting: dia.Paper.sorting.EXACT,
    ...buildPaperDefaults(),
    ...buildInteractionPaperOptions(),
    cellViewNamespace: cellNamespace,
  });

  const search = new URLSearchParams(window.location.search);
  const viewParam = search.get("view") as VisSpecView | null;
  const fixtureParam = search.get("fixture");
  const layoutParam = search.get("layout") as LayoutStrategy | null;
  const routerParam = search.get("router") as RouterStrategy | null;
  const layoutAllParam = search.get("layoutAll");
  const snapParam = search.get("snap");
  const frameParam = search.get("frame");

  const selection = resolveFixtureSelection(viewParam, fixtureParam);
  const initialView = selection.view;
  const layoutOptions = resolveLayoutOptions(initialView);
  const routerOptions = resolveRouterOptions(initialView);
  const fallbackLayout = layoutOptions[0] ?? "auto";
  const fallbackRouter = routerOptions[0] ?? "default";
  const initialLayout = layoutOptions.includes(layoutParam ?? fallbackLayout)
    ? (layoutParam ?? fallbackLayout)
    : fallbackLayout;
  const initialRouter = routerOptions.includes(routerParam ?? fallbackRouter)
    ? (routerParam ?? fallbackRouter)
    : fallbackRouter;
  const initialLayoutAll = layoutAllParam === "1" || layoutAllParam === "true";
  const snapToGrid = snapParam === "1" || snapParam === "true";
  const showFrame = frameParam === "1" || frameParam === "true";

  const fixture = selection.fixture;

  const viewSelect = document.getElementById(
    "viewSelect",
  ) as HTMLSelectElement | null;
  const layoutSelect = document.getElementById(
    "layoutSelect",
  ) as HTMLSelectElement | null;
  const layoutAllToggle = document.getElementById(
    "layoutAllToggle",
  ) as HTMLInputElement | null;
  const routerSelect = document.getElementById(
    "routerSelect",
  ) as HTMLSelectElement | null;
  const exportSvgButton = document.getElementById(
    "exportSvg",
  ) as HTMLButtonElement | null;
  const exportPngButton = document.getElementById(
    "exportPng",
  ) as HTMLButtonElement | null;

  initializeControls({
    viewSelect,
    layoutSelect,
    routerSelect,
    layoutAllToggle,
    viewOptions,
    layoutOptions,
    routerOptions,
    initialView,
    initialLayout,
    initialRouter,
    initialLayoutAll,
    resolveLayoutOptions,
    resolveRouterOptions,
  });

  if (exportSvgButton) {
    exportSvgButton.addEventListener("click", () => {
      downloadSvg(paper, fixture, buildExportName(fixture));
    });
  }

  if (exportPngButton) {
    exportPngButton.addEventListener("click", () => {
      downloadPng(paper, fixture, buildExportName(fixture));
    });
  }

  window.addEventListener("resize", () => {
    fitPaperToViewport(paper);
  });

  enableInteractionConstraints(paper, graph, { snapToGrid });
  enableZoomAndPan(paper);
  enablePortHoverLabels(paper);

  void renderCurrentDiagram();

  async function renderCurrentDiagram(): Promise<void> {
    await renderDiagram({
      graph,
      paper,
      spec: fixture,
      layoutStrategy: initialLayout,
      layoutApplyToAll: initialLayoutAll,
      routerStrategy: initialRouter,
      showFrame,
    });

    updateDiagramHeader(fixture);
    window.__SYSML_EXPORT__ = {
      svg: () => serializeSvgDocument(paper, fixture),
    };
  }
}
