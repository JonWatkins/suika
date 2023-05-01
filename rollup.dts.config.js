import { readdirSync } from "node:fs";
import dts from "rollup-plugin-dts";
import { ignoreFilter } from "./scripts/utils.js";

const packages = readdirSync("temp/packages");
const filtered = packages.filter(ignoreFilter);

export default filtered.map((pkg) => ({
  input: `./temp/packages/${pkg}/src/index.d.ts`,
  output: [{ file: `packages/${pkg}/dist/bundle.d.ts`, format: "es" }],
  external: [/\.scss$/],
  plugins: [dts()],
}));
