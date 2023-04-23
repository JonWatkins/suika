import { resolve } from "path";
import { execa } from "execa";
import { createRequire } from "module";
import { targets as allTargets } from "./utils.js";
import { rimraf } from "rimraf";
import minimist from "minimist";

const require = createRequire(import.meta.url);
const args = minimist(process.argv.slice(2));
const targets = args._;
const env = "production";

const build = async (target) => {
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
        `NODE_ENV:${env}`,
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

const buildAll = async (targets) => {
  await runParallel(targets, build);
};

const runParallel = async (source, iteratorFn) => {
  const promises = [];

  for (const item of source) {
    const p = Promise.resolve().then(() => iteratorFn(item, source));
    promises.push(p);
  }

  return Promise.all(promises);
};

const run = async () => {
  const resolvedTargets = targets.length ? targets : allTargets;
  await execa("pnpm", ["run", ["lint"]]);
  await execa("pnpm", ["run", ["format"]]);
  await buildAll(resolvedTargets);
  await execa("tsc", ["-p", ["tsconfig.build.json"]]);
  await execa("rollup", ["-c", ["rollup.dts.config.js"]]);
  await rimraf(resolve("temp"));
};

run();
