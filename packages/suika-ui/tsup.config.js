import { sassPlugin } from "esbuild-sass-plugin";
import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  splitting: true,
  sourcemap: true,
  clean: true,
  dts: true,
  format: ["esm", "cjs"],
  target: "es2020",
  bundle: true,
  esbuildPlugins: [sassPlugin({})]
});
