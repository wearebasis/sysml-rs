import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlGeometryMarker = dia.Element.define(
  "sysml.GeometryMarker",
  {
    size: { width: 24, height: 24 },
    attrs: {
      hline: {
        x1: 0,
        y1: 12,
        x2: 24,
        y2: 12,
        stroke: "var(--sysml-geometry-stroke)",
        strokeWidth: 1,
      },
      vline: {
        x1: 12,
        y1: 0,
        x2: 12,
        y2: 24,
        stroke: "var(--sysml-geometry-stroke)",
        strokeWidth: 1,
      },
      dot: {
        cx: 12,
        cy: 12,
        r: 2,
        fill: "var(--sysml-geometry-stroke)",
      },
      label: {
        fontSize: 10,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "start",
        textVerticalAnchor: "middle",
        x: 30,
        y: 12,
      },
    },
  },
  {
    markup: [
      { tagName: "line", selector: "hline" },
      { tagName: "line", selector: "vline" },
      { tagName: "circle", selector: "dot" },
      { tagName: "text", selector: "label" },
    ],
  },
);
