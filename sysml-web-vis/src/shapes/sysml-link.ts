import { dia } from "@joint/core";

export const SysmlLink = dia.Link.define(
  "sysml.Link",
  {
    attrs: {
      line: {
        connection: true,
        stroke: "var(--sysml-link-stroke)",
        strokeWidth: 1.4,
        strokeLinejoin: "round",
        targetMarker: {
          type: "path",
          d: "M 10 -5 0 0 10 5 z",
        },
      },
      wrapper: {
        connection: true,
        strokeWidth: 10,
        strokeLinejoin: "round",
      },
    },
    labels: [
      {
        attrs: {
          text: {
            fontSize: 11,
            fill: "var(--sysml-muted)",
            fontFamily: "Helvetica Neue, Helvetica, Arial, sans-serif",
          },
        },
      },
    ],
  },
  {
    markup: [
      {
        tagName: "path",
        selector: "wrapper",
        attributes: {
          fill: "none",
          cursor: "pointer",
          stroke: "transparent",
          "stroke-linecap": "round",
        },
      },
      {
        tagName: "path",
        selector: "line",
        attributes: {
          fill: "none",
          "pointer-events": "none",
        },
      },
    ],
  },
);
