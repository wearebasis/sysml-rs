import { dia } from "@joint/core";

export const SysmlActivationBar = dia.Element.define(
  "sysml.ActivationBar",
  {
    size: { width: 12, height: 70 },
    attrs: {
      body: {
        fill: "var(--sysml-sequence-fill)",
        stroke: "var(--sysml-sequence)",
        strokeWidth: 1.2,
        rx: 2,
        ry: 2,
        refWidth: "100%",
        refHeight: "100%",
      },
    },
  },
  {
    markup: [{ tagName: "rect", selector: "body" }],
  },
);
