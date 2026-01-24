import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlDecisionNode = dia.Element.define(
  "sysml.Decision",
  {
    size: { width: 46, height: 46 },
    attrs: {
      body: {
        d: "M 23 0 L 46 23 L 23 46 L 0 23 Z",
        fill: "var(--sysml-action-accent)",
        stroke: "var(--sysml-action)",
        strokeWidth: 1.4,
      },
      label: {
        fontSize: 10,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: "50%",
      },
    },
  },
  {
    markup: [
      { tagName: "path", selector: "body" },
      { tagName: "text", selector: "label" },
    ],
  },
);

export const SysmlStartNode = dia.Element.define(
  "sysml.Start",
  {
    size: { width: 22, height: 22 },
    attrs: {
      body: {
        cx: 11,
        cy: 11,
        r: 9,
        fill: "var(--sysml-action)",
        stroke: "var(--sysml-action)",
        strokeWidth: 1.4,
      },
    },
  },
  {
    markup: [{ tagName: "circle", selector: "body" }],
  },
);

export const SysmlFinalNode = dia.Element.define(
  "sysml.Final",
  {
    size: { width: 24, height: 24 },
    attrs: {
      outer: {
        cx: 12,
        cy: 12,
        r: 10,
        fill: "none",
        stroke: "var(--sysml-action)",
        strokeWidth: 2,
      },
      inner: {
        cx: 12,
        cy: 12,
        r: 6,
        fill: "var(--sysml-action)",
        stroke: "var(--sysml-action)",
      },
    },
  },
  {
    markup: [
      { tagName: "circle", selector: "outer" },
      { tagName: "circle", selector: "inner" },
    ],
  },
);

export const SysmlHistoryNode = dia.Element.define(
  "sysml.History",
  {
    size: { width: 24, height: 24 },
    attrs: {
      body: {
        cx: 12,
        cy: 12,
        r: 10,
        fill: "var(--sysml-state-fill)",
        stroke: "var(--sysml-state-stroke)",
        strokeWidth: 1.4,
      },
      label: {
        fontSize: 10,
        fontFamily: FONT,
        fill: "var(--sysml-state-stroke)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        x: 12,
        y: 12,
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

export const SysmlBarNode = dia.Element.define(
  "sysml.Bar",
  {
    size: { width: 60, height: 10 },
    attrs: {
      body: {
        x: 0,
        y: 0,
        width: 60,
        height: 10,
        fill: "var(--sysml-action)",
        stroke: "var(--sysml-action)",
        strokeWidth: 1,
        rx: 2,
        ry: 2,
      },
    },
  },
  {
    markup: [{ tagName: "rect", selector: "body" }],
  },
);
