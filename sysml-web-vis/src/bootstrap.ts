import { dia } from "@joint/core";
import { FIT_TO_CONTENT_PADDING } from "./config/constants";
import {
  fitContainerHierarchy,
  setContainerAutoFitSuspended,
} from "./interaction";
import { applyLayout, LayoutStrategy } from "./layout";
import { renderView } from "./renderers";
import { applyDiagramFrame, applyDiagramLegend } from "./renderers/frame";
import type { VisSpec } from "./vis-spec";
import type { RouterStrategy } from "./ui";

export interface DiagramRenderOptions {
  graph: dia.Graph;
  paper: dia.Paper;
  spec: VisSpec;
  layoutStrategy?: LayoutStrategy | null;
  layoutApplyToAll?: boolean;
  routerStrategy?: RouterStrategy | null;
  showFrame?: boolean;
}

export function fitPaperToViewport(paper: dia.Paper): void {
  const padding = FIT_TO_CONTENT_PADDING;
  const container = paper.el as HTMLElement | null;
  if (!container) {
    return;
  }

  const viewportWidth = container.clientWidth;
  const viewportHeight = container.clientHeight;
  if (!viewportWidth || !viewportHeight) {
    return;
  }

  paper.setDimensions(viewportWidth, viewportHeight);
  paper.scale(1, 1);
  paper.translate(0, 0);
  paper.transformToFitContent({
    padding,
    useModelGeometry: true,
    verticalAlign: "middle",
    horizontalAlign: "middle",
  });
}

export async function renderDiagram({
  graph,
  paper,
  spec,
  layoutStrategy,
  layoutApplyToAll,
  routerStrategy,
  showFrame,
}: DiagramRenderOptions): Promise<void> {
  const layoutConfig = layoutStrategy
    ? { strategy: layoutStrategy, applyToAll: layoutApplyToAll ?? false }
    : layoutApplyToAll
      ? { applyToAll: true }
      : undefined;

  setContainerAutoFitSuspended(true);
  paper.freeze();
  try {
    await applyLayout({
      graph,
      paper,
      spec,
      layoutConfig,
      render: () => renderView(graph, spec, layoutConfig),
    });

    fitContainerHierarchy(graph);
    applyDiagramFrame(graph, paper, spec, { enabled: showFrame });
    applyDiagramLegend(graph, paper, spec);

    const shouldUseAvoid = routerStrategy === "avoid";
    const shouldTryAvoid =
      shouldUseAvoid ||
      (routerStrategy === "default" && spec.view === "GeneralView");

    if (shouldTryAvoid) {
      try {
        const { enableAvoidRouter } = await import("./routers/avoid-router");
        await enableAvoidRouter(graph);
      } catch (error) {
        if (shouldUseAvoid) {
          console.warn(
            "[sysml-web-vis] Optional router libavoid-js not available.",
            error,
          );
        }
      }
    }
  } finally {
    setContainerAutoFitSuspended(false);
    paper.unfreeze();
    fitPaperToViewport(paper);
  }
}
