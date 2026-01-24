import { dia } from "@joint/core";
import { SysmlPortGroups } from "./sysml-port";

const FONT = "Helvetica Neue, Helvetica, Arial, sans-serif";

export const SysmlAction = dia.Element.define(
  "sysml.Action",
  {
    size: { width: 180, height: 64 },
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
        fill: "var(--sysml-action-accent)",
        stroke: "var(--sysml-action)",
        strokeWidth: 1.5,
        rx: 16,
        ry: 16,
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
        fontSize: 14,
        fontWeight: 600,
        fontFamily: FONT,
        fill: "var(--sysml-text)",
        textAnchor: "middle",
        textVerticalAnchor: "middle",
        refX: "50%",
        refY: 36,
      },
      icon: {
        d: "",
        fill: "none",
        stroke: "var(--sysml-action)",
        strokeWidth: 1.4,
        display: "none",
        transform: "translate(12, 12)",
      },
      divider: {
        fill: "var(--sysml-action)",
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
          height: -64,
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
      { tagName: "path", selector: "icon" },
      { tagName: "rect", selector: "divider" },
      { tagName: "text", selector: "details" },
    ],
  },
);
