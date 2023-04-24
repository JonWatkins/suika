import { targets as allTargets } from "./utils.js";
import minimist from "minimist";
import { buildTypes, bundleTypes } from "./types.js";
import { buildAll } from "./compile.js";

const args = minimist(process.argv.slice(2));
const targets = args._;

const run = async () => {
  try {
    const resolvedTargets = targets.length ? targets : allTargets;
    await buildAll(resolvedTargets);
  } finally {
    await buildTypes();
    await bundleTypes();
  }
};

run();
