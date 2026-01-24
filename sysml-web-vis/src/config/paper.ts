import { dia } from "@joint/core";

export const DEFAULT_ANCHOR: dia.Paper.Options["defaultAnchor"] = {
  name: "midSide",
  args: { useModelGeometry: true },
};

export const DEFAULT_CONNECTION_POINT: dia.Paper.Options["defaultConnectionPoint"] =
  {
    name: "boundary",
    args: { useModelGeometry: true },
  };

export function buildPaperDefaults(): Pick<
  dia.Paper.Options,
  "defaultAnchor" | "defaultConnectionPoint"
> {
  return {
    defaultAnchor: DEFAULT_ANCHOR,
    defaultConnectionPoint: DEFAULT_CONNECTION_POINT,
  };
}
