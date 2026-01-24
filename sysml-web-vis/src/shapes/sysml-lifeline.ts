import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlLifeline = dia.Element.define(
  "sysml.Lifeline",
  {
    size: { width: 160, height: 260 },
    attrs: {
      header: {
        fill: "var(--sysml-sequence-fill)",
        stroke: "var(--sysml-sequence)",
        strokeWidth: 1.4,
        rx: 10,
        ry: 10,
        refWidth: "100%",
        height: 46,
      },
      kind: {
        fontSize: 11,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 14,
      },
      label: {
        fontSize: 13,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 32,
      },
      line: {
        stroke: "var(--sysml-sequence)",
        strokeWidth: 1,
        strokeDasharray: "4 4",
      },
    },
  },
  {
    markup: [
      { tagName: "rect", selector: "header" },
      { tagName: "text", selector: "kind" },
      { tagName: "text", selector: "label" },
      { tagName: "line", selector: "line" },
    ],
  },
);
