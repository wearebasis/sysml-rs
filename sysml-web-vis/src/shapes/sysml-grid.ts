import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlGridHeader = dia.Element.define(
  "sysml.GridHeader",
  {
    size: { width: 140, height: 36 },
    attrs: {
      body: {
        fill: "var(--sysml-grid-header)",
        stroke: "var(--sysml-grid-stroke)",
        strokeWidth: 1,
        refWidth: "100%",
        refHeight: "100%",
      },
      label: {
        fontSize: 11,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: "50%",
        textWrap: {
          width: -12,
          height: -8,
          ellipsis: true,
        },
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

export const SysmlGridCell = dia.Element.define(
  "sysml.GridCell",
  {
    size: { width: 140, height: 60 },
    attrs: {
      body: {
        fill: "var(--sysml-grid-fill)",
        stroke: "var(--sysml-grid-stroke)",
        strokeWidth: 1,
        rx: 8,
        ry: 8,
        refWidth: "100%",
        refHeight: "100%",
      },
      label: {
        fontSize: 11,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: "50%",
        textWrap: {
          width: -12,
          height: -8,
          ellipsis: true,
        },
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
