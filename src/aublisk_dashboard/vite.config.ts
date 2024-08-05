import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";
import { fileURLToPath, URL } from "url";
import environment from "vite-plugin-environment";
import dotenv from "dotenv";
dotenv.config({ path: "../../.env" });

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      declaration: fileURLToPath(new URL("../declarations", import.meta.url)),
    },
  },
});
