import App from "./App.svelte";
import "$common/utils";
import "$common/variables.scss";
import { Theme, ThemeKey } from "$common/utils";

const app = new App({
	target: document.getElementById("app"),
	context: new Map([[ThemeKey, new Theme()]]),
});

export default app;
