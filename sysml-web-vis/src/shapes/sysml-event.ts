import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlEventOccurrence = dia.Element.define(
  "sysml.EventOccurrence",
  {
    size: { width: 14, height: 14 },
    attrs: {
      body: {
        cx: 7,
        cy: 7,
        r: 5,
        fill: "var(--sysml-sequence)",
        stroke: "var(--sysml-sequence)",
        strokeWidth: 1,
      },
      label: {
        fontSize: 10,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "start",
        textVerticalAnchor: "middle",
        x: 12,
        y: 7,
      },
    },
  },
  {
    markup: [
      { tagName: "circle", selector: "body" },
      { tagName: "text", selector: "label" },
    ],
  },
);
