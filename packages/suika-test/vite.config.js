import { defineConfig } from "vite";

export default defineConfig(() => {
  return {
    build: {
      outDir: "./dist",
      sourcemap: true,
      minify: false,
    },
    rollupOptions: {
        chunkFileNames: "[name].js"
    },
    optimizeDeps: {
      include: ["suika", "suika-ui", "suika-router"],
      force: true,
    },
    esbuild: {
      jsxFactory: "h",
      jsxFragment: "Fragment",
    },
  };
});
