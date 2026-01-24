import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlSequenceFragment = dia.Element.define(
  "sysml.SequenceFragment",
  {
    size: { width: 280, height: 140 },
    attrs: {
      body: {
        fill: "none",
        stroke: "var(--sysml-sequence)",
        strokeWidth: 1.2,
        strokeDasharray: "6 4",
        rx: 8,
        ry: 8,
        refWidth: "100%",
        refHeight: "100%",
      },
      operator: {
        fontSize: 11,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-sequence)",
        textAnchor: "start",
        textVerticalAnchor: "middle",
        refX: 12,
        refY: 16,
      },
    },
  },
  {
    markup: [
      { tagName: "rect", selector: "body" },
      { tagName: "text", selector: "operator" },
    ],
  },
);
