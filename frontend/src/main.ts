import App from "./App.svelte";
import { Theme, ThemeKey } from "$common/utils";

// CSS
import "$common/modern-normalize.css";
import "$common/variables.scss";

const app = new App({
	target: document.getElementById("app"),
	context: new Map([[ThemeKey, new Theme()]]),
});

export default app;
