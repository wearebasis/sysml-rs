import fs from "node:fs";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    {
      name: "jointjs-raw-svg",
      enforce: "pre",
      load(id) {
        if (id.endsWith(".svg") && id.includes("@joint/shapes-general")) {
          const svg = fs.readFileSync(id, "utf8");
          return `export default ${JSON.stringify(svg)};`;
        }
        return null;
      },
    },
  ],
  server: {
    port: Number(process.env.PORT ?? 5174),
  },
});
