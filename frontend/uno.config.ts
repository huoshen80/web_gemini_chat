import {
	defineConfig,
	presetIcons,
	presetTypography,
	presetUno,
	presetWebFonts,
} from "unocss";

export default defineConfig({
	presets: [
		presetUno(),
		presetIcons(),
		presetTypography(),
		presetWebFonts({
			provider: "bunny",
			fonts: {
				sans: "Inter:400,500,600,700",
				mono: "Fira Code",
			},
		}),
	],
	theme: {
		colors: {
			primary: {
				50: "#f5f3ff",
				100: "#ede9fe",
				200: "#ddd6fe",
				300: "#c4b5fd",
				400: "#a78bfa",
				500: "#8b5cf6",
				600: "#7c3aed",
				700: "#6d28d9",
				800: "#5b21b6",
				900: "#4c1d95",
				950: "#2e1065",
			},
		},
	},
});
