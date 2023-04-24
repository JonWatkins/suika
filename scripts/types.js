import { resolve } from "path";
import { execa } from "execa";
import { rimraf } from "rimraf";

export const buildTypes = async () => {
  try {
    await execa("tsc", ["-p", ["tsconfig.build.json"]]);
  } catch (ex) {
    // eslint ignore
  }
};

export const bundleTypes = async () => {
  try {
    await execa("rollup", ["-c", ["rollup.dts.config.js"]]);
    await rimraf(resolve("temp"));
  } catch (ex) {
    // eslint ignore
  }
};
