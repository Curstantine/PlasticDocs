<script lang="ts">
	import Router from "svelte-spa-router";

	import { PAGES, PRODUCT_NAME } from "$common/configuration";

	import LeftNav from "$lib/menu/LeftNav.svelte";
	import TopNav from "$lib/topnav/TopNav.svelte";

	export const routes = PAGES.reduce((acc, page) => {
		acc[page.path] = page.relative_local_path;
		return acc;
	}, {});
</script>

<svelte:head>
	<title>{PRODUCT_NAME}</title>
</svelte:head>

<main>
	<TopNav />

	<div class="folds">
		<LeftNav />
		<Router {routes} />
	</div>
</main>

<style lang="scss">
	main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background-color: var(--bg-1);

		.folds {
			display: flex;
			flex-direction: row;
			height: 100%;
		}
	}

	:global {
		mobile {
			display: none;

			@media only screen and (max-width: 960px) {
				display: block;
			}
		}

		.hide-mobile {
			@media only screen and (max-width: 960px) {
				display: none;
			}
		}

		html,
		body,
		#app {
			height: 100%;
		}

		button,
		input {
			border: 0;
			background: none;

			&:focus {
				outline: none;
			}
		}

		* {
			transition: all 150ms ease-in;
			font-family: var(--font-family);
			text-decoration: none;
		}
	}
</style>
