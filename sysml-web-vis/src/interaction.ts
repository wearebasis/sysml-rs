import { dia, util } from "@joint/core";
import {
  COLLAPSED_CONTAINER_SIZE,
  CONTAINER_PADDING,
} from "./config/constants";
import { fitElementToChildren } from "./auto-fit";

const containerToggleMarkup = util.svg`
  <rect @selector="button" fill="#f6f2ea" stroke="#8c7f6a" stroke-width="1" rx="2" ry="2" width="14" height="14"/>
  <path @selector="icon" fill="none" stroke="#4c3f2f" stroke-width="1.6"/>
`;

const activeContainerInteractions = new Set<string>();
const activeDragIds = new Set<string>();
const containerResizeStart = new Map<
  string,
  { width: number; height: number }
>();
let containerAutoFitSuspended = false;

export function setContainerAutoFitSuspended(suspended: boolean): void {
  containerAutoFitSuspended = suspended;
}

class ContainerToggleHighlighter extends dia.HighlighterView {
  preinitialize(): void {
    this.UPDATE_ATTRIBUTES = ["collapsed"];
    this.tagName = "g";
    this.children = containerToggleMarkup;
    this.events = {
      click: "onClick",
    };
  }

  onClick(): void {
    const element = this.cellView.model;
    if (!element.isElement()) {
      return;
    }
    toggleContainerCollapsed(element);
  }

  highlight(cellView: dia.CellView): void {
    if (this.el.childNodes.length === 0) {
      this.renderChildren();
    }

    const size = (cellView.model as dia.Element).size();
    const offsetX = size.width - 18;
    const offsetY = 12;
    this.el.setAttribute("transform", `translate(${offsetX}, ${offsetY})`);

    const collapsed = Boolean(cellView.model.get("collapsed"));
    const iconPath = collapsed ? "M 3 7 H 11 M 7 3 V 11" : "M 3 7 H 11";
    this.childNodes.icon.setAttribute("d", iconPath);
  }
}

interface InteractionOptions {
  snapToGrid?: boolean;
}

export interface ZoomPanOptions {
  minScale?: number;
  maxScale?: number;
  zoomStep?: number;
}

export function enablePortHoverLabels(paper: dia.Paper): void {
  const setLabelsVisible = (element: dia.Element, visible: boolean): void => {
    if (!element.hasPorts()) {
      return;
    }
    const display = visible ? "block" : "none";
    element.getPorts().forEach((port) => {
      element.portProp(port.id, "attrs/labelText/display", display);
    });
  };

  paper.on("element:mouseenter", (view) => {
    setLabelsVisible(view.model, true);
  });

  paper.on("element:mouseleave", (view) => {
    setLabelsVisible(view.model, false);
  });
}

export function enableInteractionConstraints(
  paper: dia.Paper,
  graph: dia.Graph,
  options: InteractionOptions = {},
): void {
  const dragState = new Map<string, { x: number; y: number }>();
  const overlapState = new Map<string, Set<string>>();
  const snapToGrid = options.snapToGrid ?? false;
  const gridSize =
    typeof paper.options.gridSize === "number" ? paper.options.gridSize : 1;

  paper.on("element:pointerdown", (view, evt, x, y) => {
    const element = view.model;
    activeDragIds.add(element.id.toString());
    dragState.set(element.id.toString(), element.position());
    if (element.isElement()) {
      overlapState.set(
        element.id.toString(),
        collectOverlappingSiblingIds(element, graph),
      );
    }
    if (element.isElement() && isContainerElement(element)) {
      if (shouldBlockContainerDrag(element, x, y)) {
        view.preventDefaultInteraction(evt);
        return;
      }
      beginContainerInteraction(element);
    }
  });

  paper.on("element:pointerup", (view) => {
    const element = view.model;
    activeDragIds.delete(element.id.toString());
    dragState.set(element.id.toString(), element.position());
    overlapState.delete(element.id.toString());
    if (element.isElement() && isContainerElement(element)) {
      endContainerInteraction(element);
    }
    if (element.isElement()) {
      updateContainerSizing(element);
    }
  });

  paper.on("element:pointermove", (view) => {
    const element = view.model;
    if (!element.isElement()) {
      return;
    }

    if (snapToGrid && gridSize > 1) {
      const snapped = snapPosition(element.position(), gridSize);
      const current = element.position();
      if (snapped.x !== current.x || snapped.y !== current.y) {
        element.position(snapped.x, snapped.y);
      }
    }

    applySequenceDragConstraints(element, dragState);

    const allowedOverlaps = overlapState.get(element.id.toString());
    if (!isPlacementAllowed(element, graph, allowedOverlaps)) {
      const previous = dragState.get(element.id.toString());
      if (previous) {
        element.position(previous.x, previous.y);
      }
      return;
    }

    dragState.set(element.id.toString(), element.position());
  });

  graph.on({
    "change:position": (cell: dia.Cell, _value, options) => {
      if (cell.isLink()) {
        return;
      }
      updateContainerSizing(cell, options);
    },
    "change:size": (cell: dia.Cell, _value, options) => {
      if (cell.isLink()) {
        return;
      }
      updateContainerSizing(cell, options);
    },
    "change:embeds": (cell: dia.Cell, _value, options) => {
      if (cell.isLink()) {
        return;
      }
      updateContainerSizing(cell, options);
    },
  });

  graph.getElements().forEach((element) => {
    attachContainerToggle(element, paper);
  });
  graph.on("add", (cell) => {
    if (cell.isElement()) {
      attachContainerToggle(cell, paper);
    }
  });
}

export function enableZoomAndPan(
  paper: dia.Paper,
  options: ZoomPanOptions = {},
): void {
  const surface = paper.el as HTMLElement | null;
  if (!surface) {
    return;
  }
  surface.style.touchAction = "none";

  const minScale = options.minScale ?? 0.2;
  const maxScale = options.maxScale ?? 2.5;
  const zoomStep = options.zoomStep ?? 0.12;

  const clampScale = (value: number): number =>
    Math.min(maxScale, Math.max(minScale, value));

  surface.addEventListener(
    "wheel",
    (event) => {
      event.preventDefault();
      const direction = event.deltaY < 0 ? 1 : -1;
      const factor = direction > 0 ? 1 + zoomStep : 1 / (1 + zoomStep);
      const currentScale = paper.matrix().a;
      const nextScale = clampScale(currentScale * factor);
      if (Math.abs(nextScale - currentScale) < 0.001) {
        return;
      }
      const anchor = paper.clientToLocalPoint(event.clientX, event.clientY);
      paper.scaleUniformAtPoint(nextScale, anchor);
    },
    { passive: false },
  );

  let isPanning = false;
  let startClient = { x: 0, y: 0 };
  let startTranslate = { tx: 0, ty: 0 };

  const beginPan = (event: dia.Event, allowLeft: boolean): void => {
    if (isPanning) {
      return;
    }
    const isMiddle = event.button === 1;
    const isLeft = event.button === 0;
    if (!isMiddle && !(allowLeft && isLeft)) {
      return;
    }
    event.preventDefault();
    isPanning = true;
    startClient = { x: event.clientX, y: event.clientY };
    startTranslate = paper.translate();
    surface.classList.add("is-panning");
  };

  paper.on("blank:pointerdown", (event: dia.Event) => {
    beginPan(event, true);
  });

  paper.on("element:pointerdown", (view, event: dia.Event) => {
    if (event.button !== 1) {
      return;
    }
    view.preventDefaultInteraction(event);
    beginPan(event, false);
  });

  const onPointerMove = (event: PointerEvent): void => {
    if (!isPanning) {
      return;
    }
    const dx = event.clientX - startClient.x;
    const dy = event.clientY - startClient.y;
    paper.translate(startTranslate.tx + dx, startTranslate.ty + dy);
  };

  const stopPan = (): void => {
    if (!isPanning) {
      return;
    }
    isPanning = false;
    surface.classList.remove("is-panning");
  };

  window.addEventListener("pointermove", onPointerMove);
  window.addEventListener("pointerup", stopPan);
  window.addEventListener("blur", stopPan);
}

function snapPosition(
  position: { x: number; y: number },
  gridSize: number,
): { x: number; y: number } {
  return {
    x: Math.round(position.x / gridSize) * gridSize,
    y: Math.round(position.y / gridSize) * gridSize,
  };
}

function applySequenceDragConstraints(
  element: dia.Element,
  dragState: Map<string, { x: number; y: number }>,
): void {
  const role = element.get("sysmlRole");
  if (role === "lifeline") {
    const start = dragState.get(element.id.toString());
    if (start) {
      element.position(element.position().x, start.y);
    }
    return;
  }

  const parent = element.getParentCell();
  if (!parent || !parent.isElement()) {
    return;
  }

  if (parent.get("sysmlRole") === "lifeline") {
    const parentBox = parent.getBBox();
    const size = element.size();
    element.position(
      parentBox.x + parentBox.width / 2 - size.width / 2,
      element.position().y,
    );
  }
}

function isContainerElement(element: dia.Element): boolean {
  return Boolean(element.get("sysmlContainer"));
}

export function buildInteractionPaperOptions(): Pick<
  dia.Paper.Options,
  "viewport" | "restrictTranslate" | "validateEmbedding" | "validateUnembedding"
> {
  return {
    viewport: (view) => {
      const cell = view.model;
      const hidden = cell
        .getAncestors()
        .some((ancestor) => Boolean(ancestor.get("collapsed")));
      return !hidden;
    },
    restrictTranslate: (elementView) => {
      const element = elementView.model;
      if (!element.isElement()) {
        return false;
      }

      const parent = element.getParentCell();
      if (!parent || !parent.isElement() || !isContainerElement(parent)) {
        return false;
      }

      if (parent.get("sysmlRestrictChildren")) {
        return buildContainerBounds(parent);
      }
      return false;
    },
    validateEmbedding: (_childView, parentView) => {
      const parent = parentView.model;
      return parent.isElement() && isContainerElement(parent);
    },
    validateUnembedding: (childView) => {
      const parent = childView.model.getParentCell();
      if (parent && parent.isElement() && isContainerElement(parent)) {
        return false;
      }
      return true;
    },
  };
}

function isCollapsedContainer(element: dia.Element): boolean {
  return Boolean(element.get("collapsed"));
}

function attachContainerToggle(element: dia.Element, paper: dia.Paper): void {
  if (!isContainerElement(element)) {
    return;
  }
  if (element.get("sysmlToggleAttached")) {
    return;
  }

  const view = paper.findViewByModel(element);
  if (!view) {
    return;
  }

  ContainerToggleHighlighter.add(view, "root", "container-toggle");
  element.set("sysmlToggleAttached", true);
}

function toggleContainerCollapsed(
  element: dia.Element,
  nextState?: boolean,
): void {
  const collapsed =
    typeof nextState === "boolean" ? nextState : !isCollapsedContainer(element);

  if (collapsed) {
    element.set("sysmlExpandedSize", element.size());
    element.set("collapsed", true);
    element.resize(
      COLLAPSED_CONTAINER_SIZE.width,
      COLLAPSED_CONTAINER_SIZE.height,
      { sysmlAutoFit: true },
    );
  } else {
    element.set("collapsed", false);
    const expanded = element.get("sysmlExpandedSize") as
      | { width: number; height: number }
      | undefined;
    if (expanded) {
      element.resize(expanded.width, expanded.height, { sysmlAutoFit: true });
    }
  }

  updateContainerSizing(element);
}

function updateContainerSizing(
  cell: dia.Cell,
  options?: dia.ModelSetOptions,
): void {
  if (!cell.isElement()) {
    return;
  }

  const cellId = cell.id.toString();
  const suppressParentFit =
    activeContainerInteractions.has(cellId) || activeDragIds.has(cellId);

  if (isContainerElement(cell) && !shouldSkipAutoFit(cell, options)) {
    fitContainerToChildren(cell);
  }

  const parent = cell.getParentCell();
  if (
    parent &&
    parent.isElement() &&
    isContainerElement(parent) &&
    !suppressParentFit &&
    !shouldSkipAutoFit(parent, options)
  ) {
    fitContainerToChildren(parent);
  }
}

function fitContainerToChildren(container: dia.Element): void {
  if (isCollapsedContainer(container)) {
    container.resize(
      COLLAPSED_CONTAINER_SIZE.width,
      COLLAPSED_CONTAINER_SIZE.height,
      { sysmlAutoFit: true },
    );
    return;
  }

  fitElementToChildren(container, {
    padding: getContainerPadding(container),
    minRect: resolveMinRect(container) ?? undefined,
  });
}

function resolveMinRect(element: dia.Element): {
  width: number;
  height: number;
} | null {
  return (
    (element.get("sysmlMinRect") as
      | { width: number; height: number }
      | undefined) ?? null
  );
}

export function fitContainerHierarchy(graph: dia.Graph): void {
  const containers = graph
    .getElements()
    .filter((element) => isContainerElement(element));
  if (containers.length === 0) {
    return;
  }

  const byDepth = containers
    .map((element) => ({
      element,
      depth: resolveContainerDepth(element),
    }))
    .sort((a, b) => b.depth - a.depth);

  byDepth.forEach(({ element }) => {
    fitContainerToChildren(element);
  });
}

function resolveContainerDepth(element: dia.Element): number {
  let depth = 0;
  let current = element.getParentCell();
  while (current && current.isElement()) {
    if (isContainerElement(current)) {
      depth += 1;
    }
    current = current.getParentCell();
  }
  return depth;
}

export function buildContainerBounds(parent: dia.Element): {
  x: number;
  y: number;
  width: number;
  height: number;
} {
  const parentBox = parent.getBBox();
  const padding = getContainerPadding(parent);
  return {
    x: parentBox.x + padding.left,
    y: parentBox.y + padding.top,
    width: Math.max(0, parentBox.width - padding.left - padding.right),
    height: Math.max(0, parentBox.height - padding.top - padding.bottom),
  };
}

function isPlacementAllowed(
  element: dia.Element,
  graph: dia.Graph,
  allowedOverlaps?: Set<string>,
): boolean {
  const overlaps = collectOverlappingSiblingIds(element, graph);
  if (!allowedOverlaps || allowedOverlaps.size === 0) {
    return overlaps.size === 0;
  }

  for (const id of overlaps) {
    if (!allowedOverlaps.has(id)) {
      return false;
    }
  }

  return true;
}

function collectOverlappingSiblingIds(
  element: dia.Element,
  graph: dia.Graph,
): Set<string> {
  const elementBox = element.getBBox();
  const siblings = resolveSiblingElements(element, graph);
  const overlaps = new Set<string>();

  siblings.forEach((other) => {
    if (boxesOverlap(elementBox, other.getBBox())) {
      overlaps.add(other.id.toString());
    }
  });

  return overlaps;
}

function resolveSiblingElements(
  element: dia.Element,
  graph: dia.Graph,
): dia.Element[] {
  const parent = element.getParentCell();
  const parentId = parent ? parent.id.toString() : null;

  return graph.getElements().filter((other) => {
    if (other.id === element.id) {
      return false;
    }

    if (isAncestor(element, other) || isAncestor(other, element)) {
      return false;
    }

    const otherParent = other.getParentCell();
    const otherParentId = otherParent ? otherParent.id.toString() : null;
    return parentId === otherParentId;
  });
}

function isAncestor(candidate: dia.Element, element: dia.Element): boolean {
  let current = element.getParentCell();
  while (current) {
    if (current.id === candidate.id) {
      return true;
    }
    current = current.getParentCell();
  }
  return false;
}

function boxesOverlap(
  a: { x: number; y: number; width: number; height: number },
  b: { x: number; y: number; width: number; height: number },
): boolean {
  const overlapX = Math.min(a.x + a.width, b.x + b.width) - Math.max(a.x, b.x);
  const overlapY =
    Math.min(a.y + a.height, b.y + b.height) - Math.max(a.y, b.y);

  return overlapX > 4 && overlapY > 4;
}

function shouldBlockContainerDrag(
  element: dia.Element,
  x: number,
  y: number,
): boolean {
  const position = element.position();
  const headerHeight = Math.min(
    getContainerPadding(element).top,
    element.size().height,
  );
  return y - position.y > headerHeight;
}

function shouldSkipAutoFit(
  element: dia.Element,
  options?: dia.ModelSetOptions,
): boolean {
  if (containerAutoFitSuspended) {
    return true;
  }
  if (options?.sysmlAutoFit) {
    return true;
  }
  return activeContainerInteractions.has(element.id.toString());
}

function beginContainerInteraction(element: dia.Element): void {
  const id = element.id.toString();
  if (!activeContainerInteractions.has(id)) {
    activeContainerInteractions.add(id);
    containerResizeStart.set(id, element.size());
  }
}

function endContainerInteraction(element: dia.Element): void {
  const id = element.id.toString();
  if (!activeContainerInteractions.has(id)) {
    return;
  }

  activeContainerInteractions.delete(id);
  const startSize = containerResizeStart.get(id);
  containerResizeStart.delete(id);

  if (!startSize) {
    return;
  }

  const currentSize = element.size();
  const resized =
    currentSize.width !== startSize.width ||
    currentSize.height !== startSize.height;
  if (!resized) {
    return;
  }

  if (!isCollapsedContainer(element)) {
    element.set("sysmlMinRect", {
      width: currentSize.width,
      height: currentSize.height,
    });
  }

  updateContainerSizing(element);
}

function getContainerPadding(element: dia.Element): {
  left: number;
  right: number;
  top: number;
  bottom: number;
} {
  return (
    (element.get("sysmlContainerPadding") as
      | { left: number; right: number; top: number; bottom: number }
      | undefined) ?? CONTAINER_PADDING
  );
}
