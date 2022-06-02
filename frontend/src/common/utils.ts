class Theme {
	public _mode: "light" | "dark";

	constructor() {
		this.mode =
			(localStorage.getItem("themeMode") as typeof this._mode) ?? "light";
	}

	public set mode(t_mode: typeof this._mode) {
		this._mode = t_mode;

		if (t_mode === "light") {
			document.documentElement.classList.value = "light";
		} else {
			document.documentElement.classList.value = "dark";
		}

		localStorage.setItem("themeMode", t_mode);
	}

	public get mode() {
		return this._mode;
	}
}

export const themeHandler = new Theme();
