import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const ignored = ["suika-test"];

export const ignoreFilter = (target) => ignored.indexOf(target) === -1;

export const targets = fs.readdirSync("packages").filter((f) => {
  if (!fs.statSync(`packages/${f}`).isDirectory()) {
    return false;
  }

  const pkg = require(`../packages/${f}/package.json`);

  if (pkg.private && !pkg.buildOptions) {
    return false;
  }

  return true;
});
