import { buildTypes, bundleTypes } from "./types.js";

const run = async () => {
  await buildTypes();
  await bundleTypes();
};

run();
