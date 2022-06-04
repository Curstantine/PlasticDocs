import type { ExtLink, Page } from "./typings";

// DOC CONFIG
export const DOC_VERSION = "0.0.1-dev";
export const DOC_FAVICON = "/favicon.ico";

// PRODUCT CONFIG
export const PRODUCT_NAME = "PlasticDocs";
export const PRODUCT_ICON = "/svelte.png"; // Relative to Public directory;
export const EXTERNAL_LINKS: ExtLink[] = [
	{
		name: "GitHub",
		url: "https://github.com/Curstantine/PlasticDocs",
		icon: { value: "/GitHub-Mark-32px.png" },
	},
	{
		name: "Discord",
		url: "https://discord.com/invite/discord",
	},
];

export const PAGES: Page[] = [];

export const routes = PAGES.reduce((acc, page) => {
	acc[page.path] = page.relative_local_path;
	return acc;
}, {});
