<script lang="ts">
	import { afterUpdate } from "svelte";
	import { fade } from "svelte/transition";

	import { forceAliveSearch, searchString } from "$common/stores";
	import type { SearchData } from "$common/typings";

	import IconButton from "$lib/button/IconButton.svelte";
	import FlexibleDrop from "$lib/extra/FlexibleDrop.svelte";
	import Spinner from "$lib/loaders/Spinner.svelte";

	import SearchBar from "./SearchBar.svelte";

	export let data: Array<SearchData>;

	$: showSpinner = !showNoContent && (!data || data.length === 0);
	$: showNoContent = $searchString.length === 0;

	const handleCloseSearch = async () => {
		$searchString = "";
		$forceAliveSearch = false;
	};

	afterUpdate(() => {
		// cleans redundent styles after an update happens.
		// should be removed when https://github.com/sveltejs/svelte/issues/7164
		// is fixed
		setTimeout(() => {
			const styles = document.head.getElementsByTagName("style");
			for (const style of styles) {
				if (style.innerHTML.length === 0) style.remove();
			}
		}, 1000);
	});
</script>

<FlexibleDrop>
	<card>
		<mobile class="action-bar">
			<IconButton on:click={handleCloseSearch}>
				<path
					d="M13.41,12l6.3-6.29a1,1,0,1,0-1.42-1.42L12,10.59,5.71,4.29A1,1,0,0,0,4.29,5.71L10.59,12l-6.3,6.29a1,1,0,0,0,0,1.42,1,1,0,0,0,1.42,0L12,13.41l6.29,6.3a1,1,0,0,0,1.42,0,1,1,0,0,0,0-1.42Z"
				/>
			</IconButton>

			<wrapper class="search">
				<SearchBar width={"100%"} mobileSupport />
			</wrapper>
		</mobile>

		<content class:no-content={showNoContent}>
			{#if showSpinner}
				<Spinner height={48} width={48} />
			{:else if showNoContent}
				<span in:fade={{ delay: 600, duration: 150 }}>
					Search something to make content magically appear!
				</span>
			{/if}
		</content>
	</card>
</FlexibleDrop>

<style lang="scss">
	card {
		display: flex;
		flex-direction: column;
		height: 10rem;
		width: 32rem;
		background-color: var(--bg-1);
		border-radius: 0.25rem;
		margin: 0.5rem 0.5rem;
		padding: 0.5rem;
		box-shadow: 0px 1px 3px rgba(0, 0, 0, 0.1),
			0px 1px 4px rgba(0, 0, 0, 0.1), 0px 1px 6px rgba(0, 0, 0, 0.1);

		@media only screen and (max-width: 960px) {
			height: calc(100% - 2rem);

			mobile.action-bar {
				display: flex;
				flex-direction: row;

				wrapper.search {
					flex: 1;
					height: 100%;
					display: flex;
					flex-direction: row;
					align-items: center;
					justify-content: end;
					margin-left: 0.5rem;
				}
			}
		}

		content {
			flex: 1;
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;

			&.no-content {
				padding: 0 2rem;
				display: none;

				@media only screen and (max-width: 960px) {
					display: flex;
				}

				span {
					color: var(--text-dark);
					font-weight: 500;
					text-align: center;
					font-size: large;
				}
			}
		}
	}
</style>
