import { dia, g, mvc, util } from "@joint/core";

const DEFAULT_PIN = 1;

async function safeImport(moduleId: string): Promise<unknown | null> {
  try {
    // @ts-expect-error Vite optional dependency
    return await import(/* @vite-ignore */ moduleId);
  } catch {
    return null;
  }
}

export interface AvoidRouterOptions {
  shapeBufferDistance?: number;
  portOverflow?: number;
  idealNudgingDistance?: number;
  commitTransactions?: boolean;
}

export async function enableAvoidRouter(
  graph: dia.Graph,
  options?: AvoidRouterOptions,
): Promise<AvoidRouter> {
  const module = await safeImport("libavoid-js");
  if (!module) {
    throw new Error("libavoid-js not available");
  }
  const { AvoidLib } = module as {
    AvoidLib: {
      load: (wasmPath?: string) => Promise<void>;
      getInstance: () => unknown;
    };
  };
  await AvoidLib.load();
  const router = new AvoidRouter(graph, AvoidLib, options);
  router.addGraphListeners();
  router.routeAll();
  return router;
}

export class AvoidRouter {
  private avoidRouter: any;
  private graphListener?: mvc.Listener;
  private margin = 0;
  private portOverflow = 0;
  private readonly graph: dia.Graph;
  private readonly AvoidLib: { getInstance: () => unknown };
  private readonly connDirections: Record<string, number>;
  private readonly shapeRefs: Record<string, any> = {};
  private readonly edgeRefs: Record<string, any> = {};
  private readonly pinIds: Record<string, number> = {};
  private readonly linksByPointer: Record<string, dia.Link> = {};
  private readonly avoidConnectorCallback: (ptr: string) => void;
  private id = 100000;
  private readonly commitTransactions: boolean;

  constructor(
    graph: dia.Graph,
    avoidLib: { getInstance: () => unknown },
    options: AvoidRouterOptions = {},
  ) {
    const Avoid = avoidLib.getInstance() as any;
    this.graph = graph;
    this.AvoidLib = avoidLib;
    this.connDirections = {
      top: Avoid.ConnDirUp,
      right: Avoid.ConnDirRight,
      bottom: Avoid.ConnDirDown,
      left: Avoid.ConnDirLeft,
      all: Avoid.ConnDirAll,
    };
    this.commitTransactions = options.commitTransactions ?? true;
    this.avoidConnectorCallback = this.onAvoidConnectorChange.bind(this);
    this.createAvoidRouter(options);
  }

  private createAvoidRouter(options: AvoidRouterOptions = {}): void {
    const Avoid = this.AvoidLib.getInstance() as any;
    const {
      shapeBufferDistance = 0,
      portOverflow = 0,
      idealNudgingDistance = 10,
    } = options;

    this.margin = shapeBufferDistance;
    this.portOverflow = portOverflow;

    const router = new Avoid.Router(Avoid.OrthogonalRouting);
    router.setRoutingParameter(
      Avoid.idealNudgingDistance,
      idealNudgingDistance,
    );
    router.setRoutingParameter(Avoid.shapeBufferDistance, shapeBufferDistance);
    router.setRoutingOption(
      Avoid.nudgeOrthogonalTouchingColinearSegments,
      false,
    );
    router.setRoutingOption(
      Avoid.performUnifyingNudgingPreprocessingStep,
      true,
    );
    router.setRoutingOption(Avoid.nudgeSharedPathsWithCommonEndPoint, true);
    router.setRoutingOption(
      Avoid.nudgeOrthogonalSegmentsConnectedToShapes,
      true,
    );

    this.avoidRouter = router;
  }

  private getAvoidRectFromElement(element: dia.Element): any {
    const Avoid = this.AvoidLib.getInstance() as any;
    const { x, y, width, height } = element.getBBox();
    return new Avoid.Rectangle(
      new Avoid.Point(x, y),
      new Avoid.Point(x + width, y + height),
    );
  }

  private getVerticesFromAvoidRoute(route: any): dia.Point[] {
    const vertices: dia.Point[] = [];
    for (let index = 1; index < route.size() - 1; index += 1) {
      const { x, y } = route.get_ps(index);
      vertices.push({ x, y });
    }
    return vertices;
  }

  private updateShape(element: dia.Element): void {
    const Avoid = this.AvoidLib.getInstance() as any;
    const shapeRect = this.getAvoidRectFromElement(element);
    if (this.shapeRefs[element.id]) {
      this.avoidRouter.moveShape(this.shapeRefs[element.id], shapeRect);
      return;
    }

    const shapeRef = new Avoid.ShapeRef(this.avoidRouter, shapeRect);
    this.shapeRefs[element.id] = shapeRef;

    const centerPin = new Avoid.ShapeConnectionPin(
      shapeRef,
      DEFAULT_PIN,
      0.5,
      0.5,
      true,
      0,
      Avoid.ConnDirAll,
    );
    centerPin.setExclusive(false);

    element.getPortGroupNames().forEach((group) => {
      const portsPositions = element.getPortsPositions(group) as Record<
        string,
        dia.Point
      >;
      const { width, height } = element.size();
      const rect = new g.Rect(0, 0, width, height);
      Object.keys(portsPositions).forEach((portId) => {
        const { x, y } = portsPositions[portId];
        const side = rect.sideNearestToPoint({ x, y });
        const pin = new Avoid.ShapeConnectionPin(
          shapeRef,
          this.getConnectionPinId(element.id.toString(), portId),
          x / width,
          y / height,
          true,
          0,
          this.connDirections[side],
        );
        pin.setExclusive(false);
      });
    });
  }

  private getConnectionPinId(elementId: string, portId: string): number {
    const pinKey = `${elementId}:${portId}`;
    if (pinKey in this.pinIds) {
      return this.pinIds[pinKey];
    }
    const pinId = this.id;
    this.id += 1;
    this.pinIds[pinKey] = pinId;
    return pinId;
  }

  private updateConnector(link: dia.Link): any {
    const Avoid = this.AvoidLib.getInstance() as any;
    const { id: sourceId, port: sourcePortId = null } = link.source();
    const { id: targetId, port: targetPortId = null } = link.target();

    if (!sourceId || !targetId) {
      this.deleteConnector(link);
      return null;
    }

    const sourceConnEnd = new Avoid.ConnEnd(
      this.shapeRefs[sourceId],
      sourcePortId
        ? this.getConnectionPinId(sourceId.toString(), sourcePortId)
        : DEFAULT_PIN,
    );
    const targetConnEnd = new Avoid.ConnEnd(
      this.shapeRefs[targetId],
      targetPortId
        ? this.getConnectionPinId(targetId.toString(), targetPortId)
        : DEFAULT_PIN,
    );

    let connRef: any;
    if (this.edgeRefs[link.id]) {
      connRef = this.edgeRefs[link.id];
    } else {
      connRef = new Avoid.ConnRef(this.avoidRouter);
      this.linksByPointer[connRef.g] = link;
    }

    connRef.setSourceEndpoint(sourceConnEnd);
    connRef.setDestEndpoint(targetConnEnd);

    if (this.edgeRefs[link.id]) {
      return connRef;
    }

    this.edgeRefs[link.id] = connRef;
    connRef.setCallback(this.avoidConnectorCallback, connRef);
    return connRef;
  }

  private deleteConnector(link: dia.Link): void {
    const connRef = this.edgeRefs[link.id];
    if (!connRef) {
      return;
    }
    this.avoidRouter.deleteConnector(connRef);
    delete this.linksByPointer[connRef.g];
    delete this.edgeRefs[link.id];
  }

  private deleteShape(element: dia.Element): void {
    const shapeRef = this.shapeRefs[element.id];
    if (!shapeRef) {
      return;
    }
    this.avoidRouter.deleteShape(shapeRef);
    delete this.shapeRefs[element.id];
  }

  private getLinkAnchorDelta(
    element: dia.Element,
    portId: string | null,
    point: dia.Point,
  ): dia.Point {
    let anchorPosition: g.Point;
    if (portId) {
      const port = element.getPort(portId);
      const portPosition = element.getPortsPositions(port.group)[portId];
      anchorPosition = new g.Point(element.position()).offset(portPosition);
    } else {
      anchorPosition = element.getBBox().center();
    }
    return point.difference(anchorPosition) as dia.Point;
  }

  private routeLink(link: dia.Link): void {
    const connRef = this.edgeRefs[link.id];
    if (!connRef) {
      return;
    }

    const route = connRef.displayRoute();
    const sourcePoint = new g.Point(route.get_ps(0));
    const targetPoint = new g.Point(route.get_ps(route.size() - 1));

    const { id: sourceId, port: sourcePortId = null } = link.source();
    const { id: targetId, port: targetPortId = null } = link.target();

    const sourceElement = link.getSourceElement();
    const targetElement = link.getTargetElement();

    if (!sourceElement || !targetElement) {
      return;
    }

    const sourceAnchorDelta = this.getLinkAnchorDelta(
      sourceElement,
      sourcePortId,
      sourcePoint,
    );
    const targetAnchorDelta = this.getLinkAnchorDelta(
      targetElement,
      targetPortId,
      targetPoint,
    );

    const linkAttributes = {
      source: {
        id: sourceId,
        port: sourcePortId || null,
        anchor: { name: "modelCenter" },
      },
      target: {
        id: targetId,
        port: targetPortId || null,
        anchor: { name: "modelCenter" },
      },
    };

    if (
      this.isRouteValid(
        route,
        sourceElement,
        targetElement,
        sourcePortId,
        targetPortId,
      )
    ) {
      linkAttributes.source.anchor.args = {
        dx: sourceAnchorDelta.x,
        dy: sourceAnchorDelta.y,
      };
      linkAttributes.target.anchor.args = {
        dx: targetAnchorDelta.x,
        dy: targetAnchorDelta.y,
      };
      link.set(
        {
          ...linkAttributes,
          vertices: this.getVerticesFromAvoidRoute(route),
          router: null,
        },
        { avoidRouter: true },
      );
    } else {
      link.set(
        {
          ...linkAttributes,
          vertices: [],
          router: {
            name: "rightAngle",
            args: { margin: this.margin - this.portOverflow },
          },
        },
        { avoidRouter: true },
      );
    }
  }

  routeAll(): void {
    this.graph.getElements().forEach((element) => this.updateShape(element));
    this.graph.getLinks().forEach((link) => this.updateConnector(link));
    this.avoidRouter.processTransaction();
  }

  private resetLink(link: dia.Link): void {
    const newAttributes = util.cloneDeep(link.attributes);
    newAttributes.vertices = [];
    newAttributes.router = null;
    delete newAttributes.source.anchor;
    delete newAttributes.target.anchor;
    link.set(newAttributes, { avoidRouter: true });
  }

  addGraphListeners(): void {
    this.removeGraphListeners();

    const listener = new mvc.Listener();
    listener.listenTo(this.graph, {
      remove: (cell: dia.Cell) => this.onCellRemoved(cell),
      add: (cell: dia.Cell) => this.onCellAdded(cell),
      change: (cell: dia.Cell, opt: dia.Cell.TransitionOptions) =>
        this.onCellChanged(cell, opt),
      reset: (_: unknown, opt: { previousModels: dia.Cell[] }) =>
        this.onGraphReset(opt.previousModels),
    });

    this.graphListener = listener;
  }

  removeGraphListeners(): void {
    this.graphListener?.stopListening();
    this.graphListener = undefined;
  }

  private onCellRemoved(cell: dia.Cell): void {
    if (cell.isElement()) {
      this.deleteShape(cell);
    } else {
      this.deleteConnector(cell);
    }
    this.avoidRouter.processTransaction();
  }

  private onCellAdded(cell: dia.Cell): void {
    if (cell.isElement()) {
      this.updateShape(cell);
    } else {
      this.updateConnector(cell);
    }
    this.avoidRouter.processTransaction();
  }

  private onCellChanged(cell: dia.Cell, opt: dia.Cell.TransitionOptions): void {
    if ((opt as { avoidRouter?: boolean }).avoidRouter) {
      return;
    }
    let needsRerouting = false;
    if ("source" in cell.changed || "target" in cell.changed) {
      if (!cell.isLink()) {
        return;
      }
      if (!this.updateConnector(cell)) {
        this.resetLink(cell);
      }
      needsRerouting = true;
    }
    if ("position" in cell.changed || "size" in cell.changed) {
      if (!cell.isElement()) {
        return;
      }
      this.updateShape(cell);
      needsRerouting = true;
    }
    if (this.commitTransactions && needsRerouting) {
      this.avoidRouter.processTransaction();
    }
  }

  private onGraphReset(previousModels: dia.Cell[]): void {
    previousModels.forEach((cell) => {
      if (cell.isElement()) {
        this.deleteShape(cell);
      } else {
        this.deleteConnector(cell);
      }
    });
    this.routeAll();
  }

  private onAvoidConnectorChange(connRefPtr: string): void {
    const link = this.linksByPointer[connRefPtr];
    if (!link) {
      return;
    }
    this.routeLink(link);
  }

  private isRouteValid(
    route: any,
    sourceElement: dia.Element,
    targetElement: dia.Element,
    sourcePortId: string | null,
    targetPortId: string | null,
  ): boolean {
    const size = route.size();
    if (size > 2) {
      return true;
    }

    const sourcePs = route.get_ps(0);
    const targetPs = route.get_ps(size - 1);
    if (sourcePs.x !== targetPs.x && sourcePs.y !== targetPs.y) {
      return false;
    }

    const margin = this.margin;
    if (
      sourcePortId &&
      targetElement.getBBox().inflate(margin).containsPoint(sourcePs)
    ) {
      return false;
    }

    if (
      targetPortId &&
      sourceElement.getBBox().inflate(margin).containsPoint(targetPs)
    ) {
      return false;
    }

    return true;
  }
}
