import { writable } from "svelte/store";

export const searchString = writable("");
export const forceAliveSearch = writable(false);
export const forceAliveMenu = writable(false);
