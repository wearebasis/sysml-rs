import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlBrowserNode = dia.Element.define(
  "sysml.BrowserNode",
  {
    size: { width: 240, height: 48 },
    attrs: {
      body: {
        fill: "var(--sysml-node-fill)",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.2,
        rx: 8,
        ry: 8,
        refWidth: "100%",
        refHeight: "100%",
      },
      icon: {
        d: "",
        fill: "none",
        stroke: "var(--sysml-browser-icon)",
        strokeWidth: 1.4,
        display: "none",
        transform: "translate(12, 12)",
      },
      kind: {
        fontSize: 9,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "start",
        textVerticalAnchor: "middle",
        refX: 40,
        refY: 14,
      },
      label: {
        fontSize: 12,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "start",
        textVerticalAnchor: "middle",
        refX: 40,
        refY: 30,
      },
    },
  },
  {
    markup: [
      { tagName: "rect", selector: "body" },
      { tagName: "path", selector: "icon" },
      { tagName: "text", selector: "kind" },
      { tagName: "text", selector: "label" },
    ],
  },
);
