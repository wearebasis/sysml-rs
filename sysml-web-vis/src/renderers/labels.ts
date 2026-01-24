import { VisSpecFlowItem, VisSpecLabel, VisSpecTransition } from "../vis-spec";
import { makeLinkLabel } from "./common";

export type LinkLabelDefinition = ReturnType<typeof makeLinkLabel>;

export interface LinkLabelOptions {
  offsetX?: number;
  offsetY?: number;
  defaultPosition?: number;
}

export function buildLinkLabels(
  labels: VisSpecLabel[] | undefined,
  options: LinkLabelOptions = {},
): LinkLabelDefinition[] {
  if (!labels || labels.length === 0) {
    return [];
  }

  const defaultPosition = options.defaultPosition ?? 0.5;
  const offsetX = options.offsetX ?? 0;

  return labels.map((label, index) =>
    makeLinkLabel(
      label.text,
      index,
      label.position ?? defaultPosition,
      offsetX,
      options.offsetY,
    ),
  );
}

export function formatTransitionLabel(transition: VisSpecTransition): string {
  const parts: string[] = [];

  if (transition.trigger) {
    parts.push(transition.trigger);
  }

  if (transition.guard) {
    parts.push(`[${transition.guard}]`);
  }

  if (transition.effect) {
    parts.push(`/ ${transition.effect}`);
  }

  return parts.join(" ");
}

export function formatFlowItemLabel(flowItem?: VisSpecFlowItem): string | null {
  if (!flowItem) {
    return null;
  }

  const parts: string[] = [];

  if (flowItem.itemLabel) {
    parts.push(flowItem.itemLabel);
  }

  if (flowItem.itemTypeId) {
    parts.push(`: ${flowItem.itemTypeId}`);
  }

  if (flowItem.multiplicity) {
    parts.push(`[${flowItem.multiplicity}]`);
  }

  if (parts.length === 0) {
    return null;
  }

  const directionTag = formatFlowDirection(flowItem.direction);
  return directionTag ? `${parts.join(" ")} ${directionTag}` : parts.join(" ");
}

function formatFlowDirection(
  direction?: VisSpecFlowItem["direction"],
): string | null {
  if (direction === "sourceToTarget") {
    return "->";
  }
  if (direction === "targetToSource") {
    return "<-";
  }
  if (direction === "bidirectional") {
    return "<->";
  }
  return null;
}

export function formatFlowItemSuffix(
  flowItem?: VisSpecFlowItem,
): string | null {
  if (!flowItem) {
    return null;
  }

  const label = flowItem.itemLabel ?? flowItem.itemTypeId;
  if (!label) {
    return null;
  }

  const multiplicity = flowItem.multiplicity
    ? ` [${flowItem.multiplicity}]`
    : "";

  return `: ${label}${multiplicity}`;
}
