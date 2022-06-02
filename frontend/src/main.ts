import App from "./App.svelte";
import { themeHandler } from "$common/utils";

const app = new App({
	target: document.getElementById("app"),
});

export default app;
