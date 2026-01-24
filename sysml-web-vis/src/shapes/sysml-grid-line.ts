import { dia } from "@joint/core";

export const SysmlGridLine = dia.Element.define(
  "sysml.GridLine",
  {
    attrs: {
      line: {
        stroke: "var(--sysml-grid-stroke)",
        strokeWidth: 1,
      },
    },
  },
  {
    markup: [{ tagName: "line", selector: "line" }],
  },
);
