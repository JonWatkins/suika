import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig(() => {
  return {
    build: {
      lib: {
        entry: resolve(__dirname, "src/index.ts"),
        name: "suika",
        fileName: "bundle",
        format: ["es", "umd"],
      },
    },
    esbuild: {
      jsxFactory: "h",
      jsxFragment: "Fragment",
    },
  };
});
