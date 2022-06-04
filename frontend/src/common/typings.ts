export type ExtLink = {
	name: string;
	url: string;
	icon?: { isRawSvg?: boolean; value: string } | string;
};

export type SearchData = {
	name: string;
	description: string;
	path: string;
};

export type Page = {
	name: string;
	path: string;
	/**
	 * Relative to $routes/
	 */
	relative_local_path: string;
};
