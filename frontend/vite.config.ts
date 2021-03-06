import { resolve } from "node:path";
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [svelte()],
	resolve: {
		alias: [
			{ find: "$lib", replacement: resolve("src", "lib") },
			{ find: "$common", replacement: resolve("src", "common") },
			{ find: "$routes", replacement: resolve("src", "routes") },
		],
	},
});
