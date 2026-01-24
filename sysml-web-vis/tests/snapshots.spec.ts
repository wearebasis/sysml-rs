import { expect, test } from "@playwright/test";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const fixturesDir = path.resolve(__dirname, "..", "fixtures");
const fixtureIds = fs
  .readdirSync(fixturesDir)
  .filter((file) => file.endsWith(".json"))
  .map((file) => path.basename(file, ".json"))
  .sort();

test.describe("VisSpec SVG snapshots", () => {
  for (const fixtureId of fixtureIds) {
    test(`snapshot ${fixtureId}`, async ({ page }) => {
      await page.goto(`/?fixture=${fixtureId}`);
      await page.waitForFunction(() => window.__SYSML_EXPORT__?.svg);
      const svg = await page.evaluate(
        () => window.__SYSML_EXPORT__?.svg() ?? "",
      );
      expect(svg).toMatchSnapshot(`${fixtureId}.svg`);
    });
  }
});
