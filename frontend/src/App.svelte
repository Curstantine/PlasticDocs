<script lang="ts">
	import Router from "svelte-spa-router";
	import LeftNav from "$lib/menu/LeftNav.svelte";
	import TopNav from "$lib/TopNav.svelte";
	import { PRODUCT_NAME, DOC_FAVICON } from "$common/constants";
	import { routes } from "$common/constants";
</script>

<svelte:head>
	<link rel="icon" href={DOC_FAVICON} />
	<title>{PRODUCT_NAME}</title>
</svelte:head>

<main>
	<Router {routes}>
		<TopNav />

		<div class="folds">
			<LeftNav />
		</div>
	</Router>
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
