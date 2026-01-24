import type { VisSpecNode, VisSpecViewMetadata } from "./vis-spec";

export type LabelingOptions = {
  kindCase: "preserve" | "title";
  nameCase: "preserve" | "definitionUsage";
};

const DEFAULT_LABELING: LabelingOptions = {
  kindCase: "preserve",
  nameCase: "preserve",
};

export function resolveLabelingOptions(
  viewMetadata?: VisSpecViewMetadata,
): LabelingOptions {
  const labeling = viewMetadata?.labeling;
  return {
    kindCase: labeling?.kindCase ?? DEFAULT_LABELING.kindCase,
    nameCase: labeling?.nameCase ?? DEFAULT_LABELING.nameCase,
  };
}

export function resolveDefinitionUsageRole(
  kind: string,
): "definition" | "usage" | null {
  const normalized = kind.toLowerCase();
  if (normalized.includes("definition")) {
    return "definition";
  }
  if (normalized.includes("usage") || normalized.includes("instance")) {
    return "usage";
  }
  return null;
}

export function shouldApplyDefinitionUsageVariant(kind: string): boolean {
  const normalized = kind.toLowerCase();
  if (!resolveDefinitionUsageRole(kind)) {
    return false;
  }

  const blocked = [
    "action",
    "state",
    "parameter",
    "pin",
    "lifeline",
    "event",
    "occurrence",
    "sequence",
    "interaction",
    "usecase",
    "use case",
    "actor",
    "decision",
    "merge",
    "fork",
    "join",
    "start",
    "final",
    "history",
    "activity",
    "control",
    "trigger",
  ];

  return !blocked.some((term) => normalized.includes(term));
}

export function formatKindLabel(
  node: VisSpecNode,
  options: LabelingOptions = DEFAULT_LABELING,
): string {
  const normalized = node.kind.toLowerCase();

  if (normalized.includes("note") || normalized.includes("comment")) {
    return "«note»";
  }

  if (normalized.includes("state") && !node.stereotype && !node.icon) {
    return "";
  }

  if (normalized.includes("actor")) {
    return "";
  }

  if (normalized.includes("usecase") || normalized.includes("use case")) {
    return "";
  }

  if (node.stereotype) {
    return node.stereotype;
  }

  const kindLabel =
    options.kindCase === "title" ? humanizeKind(node.kind) : node.kind;

  return kindLabel;
}

export function formatIconKindLabel(
  node: VisSpecNode,
  options: LabelingOptions = DEFAULT_LABELING,
): string {
  const kindLabel = formatKindLabel(node, options);
  if (!kindLabel) {
    return "";
  }
  if (kindLabel === "«note»") {
    return kindLabel;
  }
  return node.icon ? `${node.icon} ${kindLabel}` : kindLabel;
}

export function formatNodeLabel(
  node: VisSpecNode,
  options: LabelingOptions = DEFAULT_LABELING,
): string {
  const kind = node.kind.toLowerCase();

  if (kind.includes("partusage") || kind.includes("part usage")) {
    if (node.label && node.label.includes(":")) {
      return node.label;
    }
    if (node.name && node.label) {
      const name = applyNameCase(node.name, "usage", options);
      const typeLabel = applyNameCase(node.label, "definition", options);
      return `${name}: ${typeLabel}`;
    }
    const fallback = node.name ?? node.label ?? node.id;
    return applyNameCase(fallback, "usage", options);
  }

  const fallback = node.name ?? node.label ?? node.id;
  const role = resolveDefinitionUsageRole(node.kind);
  return applyNameCase(fallback, role, options);
}

function applyNameCase(
  value: string,
  role: "definition" | "usage" | null,
  options: LabelingOptions,
): string {
  if (!role || options.nameCase !== "definitionUsage") {
    return value;
  }

  return role === "definition"
    ? uppercaseFirstAlpha(value)
    : lowercaseFirstAlpha(value);
}

function uppercaseFirstAlpha(value: string): string {
  const index = findFirstAlphaIndex(value);
  if (index < 0) {
    return value;
  }

  const char = value[index];
  if (isUpper(char)) {
    return value;
  }

  return `${value.slice(0, index)}${char.toUpperCase()}${value.slice(
    index + 1,
  )}`;
}

function lowercaseFirstAlpha(value: string): string {
  const index = findFirstAlphaIndex(value);
  if (index < 0) {
    return value;
  }

  const char = value[index];
  const next = value[index + 1];
  if (isUpper(char) && next && isUpper(next)) {
    return value;
  }

  if (isLower(char)) {
    return value;
  }

  return `${value.slice(0, index)}${char.toLowerCase()}${value.slice(
    index + 1,
  )}`;
}

function findFirstAlphaIndex(value: string): number {
  for (let i = 0; i < value.length; i += 1) {
    const char = value[i];
    if (isUpper(char) || isLower(char)) {
      return i;
    }
  }
  return -1;
}

function isUpper(char: string): boolean {
  return char >= "A" && char <= "Z";
}

function isLower(char: string): boolean {
  return char >= "a" && char <= "z";
}

function humanizeKind(kind: string): string {
  return kind
    .replace(/[_-]+/g, " ")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/\s+/g, " ")
    .trim();
}
