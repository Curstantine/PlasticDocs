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
