import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

const port = 3000;

export default defineConfig({
  clearScreen: false,
  server: {
    port,
    proxy: { "/api": "http://localhost:8000" },
  },
  preview: { port },
  plugins: [tailwindcss(), sveltekit()],
});
