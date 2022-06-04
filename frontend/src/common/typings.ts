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

export type Page = NestedPage & {
	nested_routes?: NestedPage[];
};

export type NestedPage = {
	name: string;
	path: string;
	/**
	 * Relative to $routes/
	 */
	relative_local_path: string;
	item_icon_svg?: string;
};
