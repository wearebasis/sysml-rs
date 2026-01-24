import { shapes } from "@joint/core";
import { SysmlLink } from "./sysml-link";
import { SysmlActivationBar } from "./sysml-activation";
import { SysmlEventOccurrence } from "./sysml-event";
import { SysmlSequenceFragment } from "./sysml-fragment";
import { SysmlLifeline } from "./sysml-lifeline";
import { SysmlNode } from "./sysml-node";
import { SysmlBlock } from "./sysml-block";
import { SysmlRequirement } from "./sysml-requirement";
import { SysmlConstraint } from "./sysml-constraint";
import { SysmlParameter } from "./sysml-parameter";
import { SysmlUseCase } from "./sysml-usecase";
import { SysmlActor } from "./sysml-actor";
import { SysmlAction } from "./sysml-action";
import { SysmlState } from "./sysml-state";
import { SysmlPortGroups } from "./sysml-port";
import { SysmlGridCell, SysmlGridHeader } from "./sysml-grid";
import { SysmlGridLine } from "./sysml-grid-line";
import { SysmlGeometryMarker } from "./sysml-geometry";
import { SysmlPackage } from "./sysml-package";
import { SysmlDiagramFrame } from "./sysml-frame";
import { SysmlLegend } from "./sysml-legend";
import { SysmlBrowserNode } from "./sysml-browser-node";
import {
  SysmlBarNode,
  SysmlDecisionNode,
  SysmlFinalNode,
  SysmlHistoryNode,
  SysmlStartNode,
} from "./sysml-control";

export const cellNamespace = {
  ...shapes,
  sysml: {
    Link: SysmlLink,
    ActivationBar: SysmlActivationBar,
    EventOccurrence: SysmlEventOccurrence,
    SequenceFragment: SysmlSequenceFragment,
    Lifeline: SysmlLifeline,
    GridCell: SysmlGridCell,
    GridHeader: SysmlGridHeader,
    GridLine: SysmlGridLine,
    GeometryMarker: SysmlGeometryMarker,
    Block: SysmlBlock,
    Requirement: SysmlRequirement,
    Constraint: SysmlConstraint,
    Parameter: SysmlParameter,
    UseCase: SysmlUseCase,
    Actor: SysmlActor,
    Action: SysmlAction,
    State: SysmlState,
    Package: SysmlPackage,
    BrowserNode: SysmlBrowserNode,
    DiagramFrame: SysmlDiagramFrame,
    Legend: SysmlLegend,
    Node: SysmlNode,
    Decision: SysmlDecisionNode,
    Start: SysmlStartNode,
    Final: SysmlFinalNode,
    Bar: SysmlBarNode,
    History: SysmlHistoryNode,
  },
};

export {
  SysmlActivationBar,
  SysmlBarNode,
  SysmlDecisionNode,
  SysmlEventOccurrence,
  SysmlFinalNode,
  SysmlHistoryNode,
  SysmlGridCell,
  SysmlGridHeader,
  SysmlGridLine,
  SysmlGeometryMarker,
  SysmlLink,
  SysmlLifeline,
  SysmlSequenceFragment,
  SysmlBlock,
  SysmlRequirement,
  SysmlConstraint,
  SysmlParameter,
  SysmlUseCase,
  SysmlActor,
  SysmlAction,
  SysmlState,
  SysmlNode,
  SysmlPackage,
  SysmlBrowserNode,
  SysmlDiagramFrame,
  SysmlLegend,
  SysmlPortGroups,
  SysmlStartNode,
};
