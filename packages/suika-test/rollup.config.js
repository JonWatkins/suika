import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import scss from "rollup-plugin-scss";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import serve from "rollup-plugin-serve";

const plugins = [
  nodeResolve(),
  commonjs(),
  typescript({
    tsconfig: "./tsconfig.json",
    declaration: false,
    declarationMap: false,
    composite: false,
  }),
  scss({
    fileName: "bundle.css",
  }),
  terser(),
  serve({
    open: true,
    contentBase: ".",
  }),
];

export default {
  input: `src/index.ts`,
  output: {
    file: `dist/bundle.js`,
    name: "suikaTest",
    sourcemap: true,
    format: "iife",
  },
  plugins,
};
