import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlDiagramFrame = dia.Element.define(
  "sysml.DiagramFrame",
  {
    size: { width: 400, height: 300 },
    attrs: {
      body: {
        fill: "none",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.2,
        strokeDasharray: "6 4",
        rx: 6,
        ry: 6,
        refWidth: "100%",
        refHeight: "100%",
      },
      label: {
        fontSize: 12,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
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
      { tagName: "text", selector: "label" },
    ],
  },
);
