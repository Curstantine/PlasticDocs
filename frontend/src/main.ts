import App from "./App.svelte";

// CSS
import "$common/modern-normalize.css";
import { Theme, ThemeKey } from "$common/utils";
import "$common/variables.scss";

const app = new App({
	target: document.getElementById("app"),
	context: new Map([[ThemeKey, new Theme()]]),
});

export default app;
