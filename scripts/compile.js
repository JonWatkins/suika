import { resolve } from "path";
import { execa } from "execa";
import { createRequire } from "module";
import { ignoreFilter } from "./utils.js";

const require = createRequire(import.meta.url);
const { NODE_ENV } = process.env;

export const build = async (target) => {
  const pkgDir = resolve(`packages/${target}`);
  const pkg = require(`${pkgDir}/package.json`);
  const { buildOptions } = pkg;
  const { formats } = buildOptions;

  await execa(
    "rollup",
    [
      "-c",
      "--environment",
      [
        `TARGET:${target}`,
        `SOURCE_MAP:true`,
        `NODE_ENV:${NODE_ENV}`,
        `FORMATS:${formats.join("|")}`,
      ]
        .filter(Boolean)
        .join(","),
    ],
    {
      stdio: "inherit",
    }
  );
};

export const buildAll = async (targets) => {
  await runParallel(targets.filter(ignoreFilter), build);
};

export const runParallel = async (source, iteratorFn) => {
  const promises = [];

  for (const item of source) {
    const p = Promise.resolve().then(() => iteratorFn(item, source));
    promises.push(p);
  }

  return Promise.all(promises);
};
