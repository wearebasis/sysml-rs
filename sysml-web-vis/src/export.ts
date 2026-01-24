import { dia } from "@joint/core";
import { EXPORT_PADDING, SYSML_CSS_VARS } from "./config/constants";
import type { VisSpec } from "./vis-spec";

export function buildExportName(spec: VisSpec): string {
  const base = spec.viewMetadata?.title ?? spec.view;
  return base
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "");
}

export function downloadSvg(paper: dia.Paper, spec: VisSpec, name: string): void {
  const svgString = serializeSvgDocument(paper, spec);
  downloadBlob(svgString, "image/svg+xml", `${name || "diagram"}.svg`);
}

export function downloadPng(paper: dia.Paper, spec: VisSpec, name: string): void {
  const { svg, width, height, x, y } = buildSvgDocument(paper, spec);
  svg.setAttribute("viewBox", `${x} ${y} ${width} ${height}`);
  svg.setAttribute("width", `${width}`);
  svg.setAttribute("height", `${height}`);

  const serializer = new XMLSerializer();
  const svgString = serializer.serializeToString(svg);
  const blob = new Blob([svgString], { type: "image/svg+xml" });
  const url = URL.createObjectURL(blob);
  const img = new Image();
  img.onload = () => {
    const dpr = window.devicePixelRatio || 1;
    const canvas = document.createElement("canvas");
    canvas.width = Math.ceil(width * dpr);
    canvas.height = Math.ceil(height * dpr);
    const ctx = canvas.getContext("2d");
    if (!ctx) {
      URL.revokeObjectURL(url);
      return;
    }

    ctx.scale(dpr, dpr);
    ctx.fillStyle = resolveBackgroundColor();
    ctx.fillRect(0, 0, width, height);
    ctx.drawImage(img, 0, 0, width, height);

    canvas.toBlob((png) => {
      if (png) {
        downloadBlob(png, "image/png", `${name || "diagram"}.png`);
      }
      URL.revokeObjectURL(url);
    });
  };
  img.onerror = () => {
    URL.revokeObjectURL(url);
  };
  img.src = url;
}

export function serializeSvgDocument(paper: dia.Paper, spec: VisSpec): string {
  const { svg, width, height, x, y } = buildSvgDocument(paper, spec);
  svg.setAttribute("viewBox", `${x} ${y} ${width} ${height}`);
  svg.setAttribute("width", `${width}`);
  svg.setAttribute("height", `${height}`);
  const serializer = new XMLSerializer();
  return serializer.serializeToString(svg);
}

function buildSvgDocument(
  paper: dia.Paper,
  spec: VisSpec,
): { svg: SVGSVGElement; width: number; height: number; x: number; y: number } {
  const svg = paper.svg.cloneNode(true) as SVGSVGElement;
  svg.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  svg.setAttribute("xmlns:xlink", "http://www.w3.org/1999/xlink");

  const style = document.createElementNS("http://www.w3.org/2000/svg", "style");
  style.textContent = buildSvgStyleVariables();
  svg.insertBefore(style, svg.firstChild);

  const bbox = paper.getContentBBox({ useModelGeometry: true });
  const padding = EXPORT_PADDING;
  const width = Math.max(1, bbox.width + padding * 2);
  const height = Math.max(1, bbox.height + padding * 2);
  const x = bbox.x - padding;
  const y = bbox.y - padding;

  const background = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "rect",
  );
  background.setAttribute("x", `${x}`);
  background.setAttribute("y", `${y}`);
  background.setAttribute("width", `${width}`);
  background.setAttribute("height", `${height}`);
  background.setAttribute("fill", resolveBackgroundColor());
  svg.insertBefore(background, svg.firstChild);

  if (spec.viewMetadata?.title) {
    svg.setAttribute("data-title", spec.viewMetadata.title);
  }

  return { svg, width, height, x, y };
}

function downloadBlob(
  data: Blob | string,
  type: string,
  filename: string,
): void {
  const blob = typeof data === "string" ? new Blob([data], { type }) : data;
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  link.click();
  URL.revokeObjectURL(url);
}

function resolveBackgroundColor(): string {
  const root = getComputedStyle(document.documentElement);
  const color = root.getPropertyValue("--sysml-bg").trim();
  return color || "#ffffff";
}

function buildSvgStyleVariables(): string {
  const root = getComputedStyle(document.documentElement);
  const vars = SYSML_CSS_VARS.map((name) => {
    const value = root.getPropertyValue(name).trim();
    return value ? `${name}: ${value};` : "";
  })
    .filter(Boolean)
    .join(" ");

  return `svg { ${vars} }`;
}
