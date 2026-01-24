import { dia } from "@joint/core";
import { SysmlPortGroups } from "./sysml-port";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlParameter = dia.Element.define(
  "sysml.Parameter",
  {
    size: { width: 120, height: 40 },
    ports: {
      groups: SysmlPortGroups,
      markup: [
        { tagName: "rect", selector: "portBody" },
        { tagName: "line", selector: "portDecoration" },
      ],
    },
    portLabelMarkup: [{ tagName: "text", selector: "labelText" }],
    attrs: {
      body: {
        fill: "var(--sysml-param-fill)",
        stroke: "var(--sysml-action)",
        strokeWidth: 1.4,
        rx: 12,
        ry: 12,
        refWidth: "100%",
        refHeight: "100%",
      },
      kind: {
        fontSize: 10,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 12,
      },
      label: {
        fontSize: 12,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 26,
      },
      divider: {
        fill: "var(--sysml-action)",
        opacity: 0.35,
        refX: 0,
        refY: 30,
        refWidth: "100%",
        height: 1,
        display: "none",
      },
      details: {
        fontSize: 10,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "start",
        textVerticalAnchor: "top",
        refX: 12,
        refY: 34,
        textWrap: {
          width: -24,
          height: -40,
          ellipsis: true,
        },
      },
    },
  },
  {
    markup: [
      { tagName: "rect", selector: "body" },
      { tagName: "text", selector: "kind" },
      { tagName: "text", selector: "label" },
      { tagName: "rect", selector: "divider" },
      { tagName: "text", selector: "details" },
    ],
  },
);
