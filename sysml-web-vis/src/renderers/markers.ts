export type MarkerKind = "none" | "triangle" | "diamond" | "open";

type MarkerStyle = {
  stroke?: string;
  fill?: string;
  strokeWidth?: number;
  refX?: number;
};

const MARKER_PATHS: Record<Exclude<MarkerKind, "none">, string> = {
  triangle: "M 10 -5 0 0 10 5 z",
  open: "M 10 -5 0 0 10 5",
  diamond: "M 6 0 L 12 6 L 6 12 L 0 6 z",
};

export function buildMarker(
  kind: MarkerKind,
  stroke?: string,
  style?: MarkerStyle,
): Record<string, unknown> {
  if (kind === "none") {
    return { type: "path", d: "" };
  }

  const path = MARKER_PATHS[kind];
  const resolvedStroke = style?.stroke ?? stroke ?? "var(--sysml-link-stroke)";
  const resolvedFill =
    style?.fill ?? (kind === "open" ? "none" : resolvedStroke);
  const resolvedStrokeWidth = style?.strokeWidth ?? 1;
  const resolvedRefX = style?.refX ?? (kind === "diamond" ? 12 : 10);

  return {
    type: "path",
    d: path,
    refX: resolvedRefX,
    stroke: resolvedStroke,
    strokeWidth: resolvedStrokeWidth,
    fill: resolvedFill,
  };
}
