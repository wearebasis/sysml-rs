import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlActor = dia.Element.define(
  "sysml.Actor",
  {
    size: { width: 80, height: 120 },
    attrs: {
      head: {
        cx: "calc(w/2)",
        cy: "calc(h/6)",
        r: "calc(w/8)",
        fill: "none",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.4,
      },
      torso: {
        x1: "calc(w/2)",
        y1: "calc(h/4)",
        x2: "calc(w/2)",
        y2: "calc(h/1.5)",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.4,
      },
      arms: {
        x1: "calc(w/4)",
        y1: "calc(h/2)",
        x2: "calc(w/1.3333)",
        y2: "calc(h/2)",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.4,
      },
      legLeft: {
        x1: "calc(w/2)",
        y1: "calc(h/1.5)",
        x2: "calc(w/3)",
        y2: "calc(h/1.2)",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.4,
      },
      legRight: {
        x1: "calc(w/2)",
        y1: "calc(h/1.5)",
        x2: "calc(w/1.5)",
        y2: "calc(h/1.2)",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.4,
      },
      label: {
        fontSize: 12,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        x: "calc(w/2)",
        y: "calc(h-10)",
        textWrap: {
          width: -10,
          height: 20,
          ellipsis: true,
        },
      },
    },
  },
  {
    markup: [
      { tagName: "circle", selector: "head" },
      { tagName: "line", selector: "torso" },
      { tagName: "line", selector: "arms" },
      { tagName: "line", selector: "legLeft" },
      { tagName: "line", selector: "legRight" },
      { tagName: "text", selector: "label" },
    ],
  },
);
