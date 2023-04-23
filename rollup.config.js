import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import scss from "rollup-plugin-scss";

const { FORMATS, TARGET } = process.env;
const formats = FORMATS.split("|");

const snakeToCamel = (str) =>
  str
    .toLowerCase()
    .replace(/([-_][a-z])/g, (group) =>
      group.toUpperCase().replace("-", "").replace("_", "")
    );

const plugins = [
  typescript({
    tsconfig: "./tsconfig.base.json",
    declaration: false,
    declarationMap: false,
    composite: false,
  }),
  scss({
    fileName: "bundle.css",
  }),
  terser(),
];

export default formats.map((format) => {
  const config = {
    input: `packages/${TARGET}/src/index.ts`,
    output: {
      file: `packages/${TARGET}/dist/bundle.${format}.js`,
      name: snakeToCamel(TARGET),
      format,
    },
    plugins,
  };

  if (TARGET !== "suika") {
    config.external = ["suika"];
    config.output.globals = {
      suika: "suika",
    };
  }

  return config;
});
