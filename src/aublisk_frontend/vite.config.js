import { fileURLToPath, URL } from "url";
import vue from "@vitejs/plugin-vue"
import { defineConfig } from "vite";
import environment from "vite-plugin-environment";
import dotenv from "dotenv";
import {viteStaticCopy} from "vite-plugin-static-copy"

dotenv.config({ path: "../../.env" });
import path from "path"
export default defineConfig({
  plugins: [
    vue(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
    viteStaticCopy({
      targets: [
        {
          src: 'static/*',
          dest: 'static'
        }
      ]
    })
  ],
  build: {
    emptyOutDir: true,
    rollupOptions: {
      input: ["index.html", "list.html", "project.html", "404.html"]
    }
  },
  assetsDir: '',
  emptyOutDir: true,
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./"),
      declaration: fileURLToPath(new URL("../declarations", import.meta.url)),
    },
  },
});
