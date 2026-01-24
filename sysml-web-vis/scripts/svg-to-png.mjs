import { chromium } from "@playwright/test";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const snapshotsDir = path.join(root, "tests", "snapshots.spec.ts-snapshots");
const outputDir = path.join(root, "tests", "png");

if (!fs.existsSync(snapshotsDir)) {
  console.error(`Missing snapshot directory: ${snapshotsDir}`);
  process.exit(1);
}

const svgFiles = fs
  .readdirSync(snapshotsDir)
  .filter((file) => file.endsWith(".svg"));

if (svgFiles.length === 0) {
  console.error("No SVG snapshots found to convert.");
  process.exit(1);
}

fs.mkdirSync(outputDir, { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage();

for (const file of svgFiles) {
  const svgPath = path.join(snapshotsDir, file);
  const svg = fs.readFileSync(svgPath, "utf8");
  const { width, height } = parseSvgSize(svg);

  await page.setViewportSize({ width, height });
  await page.setContent(wrapSvg(svg), { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(50);

  const outputPath = path.join(outputDir, file.replace(/\.svg$/, ".png"));
  await page.screenshot({ path: outputPath, omitBackground: false });
  console.log(`Wrote ${outputPath}`);
}

await browser.close();

function wrapSvg(svg) {
  return `<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <style>
      html, body {
        margin: 0;
        padding: 0;
        background: transparent;
      }
      svg {
        display: block;
      }
    </style>
  </head>
  <body>${svg}</body>
</html>`;
}

function parseSvgSize(svg) {
  const width = parseNumber(svg.match(/width="([\d.]+)(px)?"/i));
  const height = parseNumber(svg.match(/height="([\d.]+)(px)?"/i));

  if (width && height) {
    return { width: Math.ceil(width), height: Math.ceil(height) };
  }

  const viewBoxMatch = svg.match(/viewBox="[\d.\-]+\s+[\d.\-]+\s+([\d.]+)\s+([\d.]+)"/i);
  if (viewBoxMatch) {
    return {
      width: Math.ceil(Number.parseFloat(viewBoxMatch[1])),
      height: Math.ceil(Number.parseFloat(viewBoxMatch[2])),
    };
  }

  return { width: 1200, height: 800 };
}

function parseNumber(match) {
  if (!match) {
    return null;
  }

  const value = Number.parseFloat(match[1]);
  return Number.isFinite(value) ? value : null;
}
