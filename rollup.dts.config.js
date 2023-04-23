import { readdirSync } from "node:fs";
import dts from "rollup-plugin-dts";
const packages = readdirSync("temp/packages");

export default packages.map((pkg) => ({
  input: `./temp/packages/${pkg}/src/index.d.ts`,
  output: [{ file: `packages/${pkg}/dist/bundle.d.ts`, format: "es" }],
  external: [/\.scss$/],
  plugins: [dts()],
}));
