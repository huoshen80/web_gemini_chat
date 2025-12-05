import { resolve } from "node:path";
import react from "@vitejs/plugin-react";
import UnoCSS from "unocss/vite";
import { defineConfig } from "vite";

// https://vite.dev/config/
export default defineConfig({
	plugins: [react(), UnoCSS()],

	resolve: {
		// 设置文件./src路径为 @
		alias: [
			{
				find: "@",
				replacement: resolve(__dirname, "./src"),
			},
		],
	},
});
