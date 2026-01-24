import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlLegend = dia.Element.define(
  "sysml.Legend",
  {
    size: { width: 220, height: 120 },
    attrs: {
      body: {
        fill: "var(--sysml-panel)",
        stroke: "var(--sysml-panel-border)",
        strokeWidth: 1,
        rx: 10,
        ry: 10,
        refWidth: "100%",
        refHeight: "100%",
      },
      title: {
        fontSize: 12,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "start",
        textVerticalAnchor: "middle",
        refX: 12,
        refY: 18,
      },
      items: {
        fontSize: 11,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "start",
        textVerticalAnchor: "top",
        refX: 12,
        refY: 36,
        textWrap: {
          width: -24,
          height: -44,
          ellipsis: true,
        },
      },
    },
  },
  {
    markup: [
      { tagName: "rect", selector: "body" },
      { tagName: "text", selector: "title" },
      { tagName: "text", selector: "items" },
    ],
  },
);
