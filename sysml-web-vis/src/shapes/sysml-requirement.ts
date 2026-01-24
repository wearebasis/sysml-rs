import { dia } from "@joint/core";
import { SysmlPortGroups } from "./sysml-port";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlRequirement = dia.Element.define(
  "sysml.Requirement",
  {
    size: { width: 190, height: 72 },
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
        fill: "var(--sysml-req-fill)",
        stroke: "var(--sysml-req-stroke)",
        strokeWidth: 1.5,
        rx: 4,
        ry: 4,
        refWidth: "100%",
        refHeight: "100%",
      },
      meta: {
        fontSize: 9,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 8,
        display: "none",
        textWrap: {
          width: -24,
          height: 12,
          ellipsis: true,
        },
      },
      kind: {
        fontSize: 11,
        fontFamily: FONT,
        fill: "var(--sysml-muted)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 16,
      },
      label: {
        fontSize: 13,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 36,
      },
      divider: {
        fill: "var(--sysml-req-stroke)",
        opacity: 0.35,
        refX: 0,
        refY: 50,
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
        refY: 56,
        textWrap: {
          width: -24,
          height: -70,
          ellipsis: true,
        },
      },
    },
  },
  {
    markup: [
      { tagName: "rect", selector: "body" },
      { tagName: "text", selector: "meta" },
      { tagName: "text", selector: "kind" },
      { tagName: "text", selector: "label" },
      { tagName: "rect", selector: "divider" },
      { tagName: "text", selector: "details" },
    ],
  },
);
