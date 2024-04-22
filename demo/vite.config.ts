import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  build: {
    rollupOptions: {
      output: {
        manualChunks(id: string) {
          if (id.match("dnd-kit")) {
            return "dnd-kit";
          }
          if (id.match("zod")) {
            return "zod";
          }
          if (id.match("react-colorful")) {
            return "react-colorful";
          }
        },
      },
    },
  },
});
