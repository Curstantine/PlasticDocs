<script lang="ts">
	import LeftNav from "$lib/LeftNav.svelte";
	import TopNav from "$lib/TopNav.svelte";
	import { PRODUCT_NAME, DOC_FAVICON } from "$common/constants";
</script>

<svelte:head>
	<link rel="icon" href={DOC_FAVICON} />
	<title>{PRODUCT_NAME}</title>
</svelte:head>

<main>
	<TopNav />

	<div class="folds">
		<LeftNav />
	</div>
</main>

<style lang="scss">
	main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;

		.folds {
			display: flex;
			flex-direction: row;
			height: 100%;
		}
	}

	:global {
		.light {
			// background colors
			--bg-1: rgb(255, 255, 255);
			--bg-2: rgb(248, 249, 252);
			--bg-3: rgb(237, 239, 244);
			--bg-accent: rgb(252, 220, 209);
			// end

			// text config
			--font-family: "Nunito Sans", sans-serif;
			--text-lighter: rgb(113, 127, 154);
			--text-light: rgb(83, 87, 107);
			--text-dark: rgb(47, 51, 72);
			--text-accent: rgb(255, 62, 0);
			// end

			// misc
			--red: rgb(229, 62, 62);
			--border: rgb(237, 239, 244);
			// end
		}

		// html,
		// body,
		// #app {
		// 	margin: 0;
		// 	height: 100%;
		// }

		// defaults
		button,
		input {
			border: 0;

			&:focus {
				outline: none;
			}
		}

		* {
			font-family: var(--font-family);
			text-decoration: none;
		}
	}
</style>
