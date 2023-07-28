import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import scss from "rollup-plugin-scss";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import serve from "rollup-plugin-serve";
import postcss from "postcss";
import autoprefixer from "autoprefixer";
import purgecss from "@fullhuman/postcss-purgecss";

const purgeOpts = {
  defaultExtractor: (content) => content.match(/[\w-/:.]+(?<!:.)/g) || [],
  content: [
    "./**/*.html",
    "./**/*.{ts,tsx}",
    "./node_modules/suika/**/*.{js,css}",
    "./node_modules/suika-ui/**/*.{js,css}",
    "./node_modules/suika-router/**/*.{js,css}",
  ],
};

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
    processor: () => postcss([autoprefixer, purgecss(purgeOpts)]),
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
