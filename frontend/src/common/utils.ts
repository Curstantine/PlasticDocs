export class Theme {
	public mode: "light" | "dark";

	constructor() {
		this.setMode(
			(localStorage.getItem("themeMode") as typeof this.mode) ?? "light"
		);
	}

	public setMode(mode: typeof this.mode) {
		this.mode = mode;

		if (mode === "light") {
			document.documentElement.classList.value = "light";
		} else {
			document.documentElement.classList.value = "dark";
		}

		localStorage.setItem("themeMode", mode);
	}

	public invert() {
		this.setMode(this.mode === "light" ? "dark" : "light");
	}
}

export const ThemeKey = Symbol();
