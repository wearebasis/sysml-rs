import { dia } from "@joint/core";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlUseCase = dia.Element.define(
  "sysml.UseCase",
  {
    size: { width: 160, height: 70 },
    attrs: {
      body: {
        cx: "calc(w/2)",
        cy: "calc(h/2)",
        rx: "calc(w/2)",
        ry: "calc(h/2)",
        fill: "var(--sysml-node-fill)",
        stroke: "var(--sysml-node-stroke)",
        strokeWidth: 1.4,
      },
      label: {
        fontSize: 13,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        x: "calc(w/2)",
        y: "calc(h/2)",
        textWrap: {
          width: -16,
          height: -12,
          ellipsis: true,
        },
      },
    },
  },
  {
    markup: [
      { tagName: "ellipse", selector: "body" },
      { tagName: "text", selector: "label" },
    ],
  },
);
