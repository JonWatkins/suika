import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig(() => {
  return {
    build: {
      lib: {
        entry: resolve(__dirname, "src/index.ts"),
        name: "NekoJS",
        fileName: "bundle",
      },
    },
    esbuild: {
      jsxFactory: "h",
      jsxFragment: "Fragment",
    },
  };
});
