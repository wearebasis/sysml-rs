import { dia } from "@joint/core";
import { LayoutConfig } from "../layout";
import { renderActionFlowView } from "./action-flow";
import { renderBrowserView } from "./browser";
import { renderGeneralView } from "./general";
import { renderGeometryView } from "./geometry";
import { renderGridView } from "./grid";
import { renderInterconnectionView } from "./interconnection";
import { renderSequenceView } from "./sequence";
import { renderStateTransitionView } from "./state-transition";
import { VisSpec } from "../vis-spec";

export function renderView(
  graph: dia.Graph,
  spec: VisSpec,
  layoutConfig?: LayoutConfig,
): void {
  switch (spec.view) {
    case "GeneralView":
      renderGeneralView(graph, spec, layoutConfig);
      return;
    case "InterconnectionView":
      renderInterconnectionView(graph, spec, layoutConfig);
      return;
    case "StateTransitionView":
      renderStateTransitionView(graph, spec, layoutConfig);
      return;
    case "ActionFlowView":
      renderActionFlowView(graph, spec, layoutConfig);
      return;
    case "SequenceView":
      renderSequenceView(graph, spec, layoutConfig);
      return;
    case "BrowserView":
      renderBrowserView(graph, spec, layoutConfig);
      return;
    case "GridView":
      renderGridView(graph, spec, layoutConfig);
      return;
    case "GeometryView":
      renderGeometryView(graph, spec, layoutConfig);
      return;
    default: {
      const exhaustive: never = spec.view;
      throw new Error(`Unsupported view ${exhaustive}`);
    }
  }
}
