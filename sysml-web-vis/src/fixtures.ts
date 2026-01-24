import type { VisSpec, VisSpecView } from "./vis-spec";
import generalFixture from "../fixtures/general.json";
import generalBddFixture from "../fixtures/general-bdd.json";
import generalRequirementsFixture from "../fixtures/general-requirements.json";
import generalPackageFixture from "../fixtures/general-package.json";
import generalUsecaseFixture from "../fixtures/general-usecase.json";
import interconnectionFixture from "../fixtures/interconnection.json";
import interconnectionIbdFixture from "../fixtures/interconnection-ibd.json";
import interconnectionParametricFixture from "../fixtures/interconnection-parametric.json";
import stateTransitionFixture from "../fixtures/state-transition.json";
import actionFlowFixture from "../fixtures/action-flow.json";
import sequenceFixture from "../fixtures/sequence.json";
import browserFixture from "../fixtures/browser.json";
import gridFixture from "../fixtures/grid.json";
import geometryFixture from "../fixtures/geometry.json";

export const DEFAULT_VIEW: VisSpecView = "GeneralView";

export type FixtureId =
  | "general"
  | "general-bdd"
  | "general-requirements"
  | "general-package"
  | "general-usecase"
  | "interconnection"
  | "interconnection-ibd"
  | "interconnection-parametric"
  | "state-transition"
  | "action-flow"
  | "sequence"
  | "browser"
  | "grid"
  | "geometry";

export interface FixtureDefinition {
  id: FixtureId;
  view: VisSpecView;
  label: string;
  diagramTypes: string[];
  spec: VisSpec;
}

export const fixtureByView: Record<VisSpecView, VisSpec> = {
  GeneralView: generalFixture as VisSpec,
  InterconnectionView: interconnectionFixture as VisSpec,
  StateTransitionView: stateTransitionFixture as VisSpec,
  ActionFlowView: actionFlowFixture as VisSpec,
  SequenceView: sequenceFixture as VisSpec,
  BrowserView: browserFixture as VisSpec,
  GridView: gridFixture as VisSpec,
  GeometryView: geometryFixture as VisSpec,
};

export const viewOptions: VisSpecView[] = [
  "GeneralView",
  "InterconnectionView",
  "StateTransitionView",
  "ActionFlowView",
  "SequenceView",
  "BrowserView",
  "GridView",
  "GeometryView",
];

export const fixtureCatalog: FixtureDefinition[] = [
  {
    id: "general",
    view: "GeneralView",
    label: "General (mixed)",
    diagramTypes: ["BDD", "Package", "Requirement", "Use Case"],
    spec: generalFixture as VisSpec,
  },
  {
    id: "general-bdd",
    view: "GeneralView",
    label: "Block Definition Diagram",
    diagramTypes: ["BDD"],
    spec: generalBddFixture as VisSpec,
  },
  {
    id: "general-requirements",
    view: "GeneralView",
    label: "Requirement Diagram",
    diagramTypes: ["Requirement"],
    spec: generalRequirementsFixture as VisSpec,
  },
  {
    id: "general-package",
    view: "GeneralView",
    label: "Package Diagram",
    diagramTypes: ["Package"],
    spec: generalPackageFixture as VisSpec,
  },
  {
    id: "general-usecase",
    view: "GeneralView",
    label: "Use Case Diagram",
    diagramTypes: ["Use Case"],
    spec: generalUsecaseFixture as VisSpec,
  },
  {
    id: "interconnection",
    view: "InterconnectionView",
    label: "Interconnection (mixed)",
    diagramTypes: ["IBD", "Parametric"],
    spec: interconnectionFixture as VisSpec,
  },
  {
    id: "interconnection-ibd",
    view: "InterconnectionView",
    label: "Internal Block Diagram",
    diagramTypes: ["IBD"],
    spec: interconnectionIbdFixture as VisSpec,
  },
  {
    id: "interconnection-parametric",
    view: "InterconnectionView",
    label: "Parametric Diagram",
    diagramTypes: ["Parametric"],
    spec: interconnectionParametricFixture as VisSpec,
  },
  {
    id: "action-flow",
    view: "ActionFlowView",
    label: "Activity Diagram",
    diagramTypes: ["Activity"],
    spec: actionFlowFixture as VisSpec,
  },
  {
    id: "state-transition",
    view: "StateTransitionView",
    label: "State Machine Diagram",
    diagramTypes: ["State Machine"],
    spec: stateTransitionFixture as VisSpec,
  },
  {
    id: "sequence",
    view: "SequenceView",
    label: "Sequence Diagram",
    diagramTypes: ["Sequence"],
    spec: sequenceFixture as VisSpec,
  },
  {
    id: "geometry",
    view: "GeometryView",
    label: "Geometry Diagram",
    diagramTypes: ["Geometry"],
    spec: geometryFixture as VisSpec,
  },
  {
    id: "browser",
    view: "BrowserView",
    label: "Browser (non-diagram)",
    diagramTypes: ["Non-diagram"],
    spec: browserFixture as VisSpec,
  },
  {
    id: "grid",
    view: "GridView",
    label: "Grid (non-diagram)",
    diagramTypes: ["Non-diagram"],
    spec: gridFixture as VisSpec,
  },
];

const fixtureById = new Map(
  fixtureCatalog.map((fixture) => [fixture.id, fixture]),
);

export function resolveView(view?: VisSpecView | null): VisSpecView {
  if (view && fixtureByView[view]) {
    return view;
  }
  return DEFAULT_VIEW;
}

export function resolveFixture(
  view?: VisSpecView | null,
  fixtureId?: string | null,
): VisSpec {
  const resolvedFixture = resolveFixtureId(fixtureId);
  if (resolvedFixture) {
    return resolvedFixture.spec;
  }

  const resolved = resolveView(view);
  return fixtureByView[resolved];
}

export function resolveFixtureSelection(
  view?: VisSpecView | null,
  fixtureId?: string | null,
): { view: VisSpecView; fixture: VisSpec; fixtureId?: FixtureId } {
  const resolvedFixture = resolveFixtureId(fixtureId);
  if (resolvedFixture) {
    return {
      view: resolvedFixture.view,
      fixture: resolvedFixture.spec,
      fixtureId: resolvedFixture.id,
    };
  }

  const resolvedView = resolveView(view);
  return {
    view: resolvedView,
    fixture: fixtureByView[resolvedView],
  };
}

export function resolveFixtureId(
  fixtureId?: string | null,
): FixtureDefinition | undefined {
  if (!fixtureId) {
    return undefined;
  }

  return fixtureById.get(fixtureId as FixtureId);
}
