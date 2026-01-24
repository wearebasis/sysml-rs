import { dia } from "@joint/core";
import { VisSpecNode, VisSpecPoint, VisSpecSize } from "../vis-spec";

export interface ControlNodeDefinition {
  matches(kind: string): boolean;
  create(): dia.Element;
  afterCreate?(element: dia.Element, node: VisSpecNode): void;
}

export function positionElement(
  element: dia.Element,
  node: VisSpecNode,
  position?: VisSpecPoint,
  cellSize?: VisSpecSize,
): dia.Element {
  element.set("id", node.id);
  const size = node.size ?? element.size();
  element.resize(size.width, size.height);

  const anchor = position ?? { x: 40, y: 40 };
  const offsetX = cellSize ? Math.max(0, (cellSize.width - size.width) / 2) : 0;
  const offsetY = cellSize
    ? Math.max(0, (cellSize.height - size.height) / 2)
    : 0;
  element.position(anchor.x + offsetX, anchor.y + offsetY);
  return element;
}

export function createControlNode(
  node: VisSpecNode,
  definitions: ControlNodeDefinition[],
  position?: VisSpecPoint,
  cellSize?: VisSpecSize,
): dia.Element | null {
  const kind = node.kind.toLowerCase();
  for (const definition of definitions) {
    if (!definition.matches(kind)) {
      continue;
    }
    const element = positionElement(
      definition.create(),
      node,
      position,
      cellSize,
    );
    definition.afterCreate?.(element, node);
    return element;
  }

  return null;
}
