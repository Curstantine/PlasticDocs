<script lang="ts">
	import {
		forceAliveMenu,
		forceAliveSearch,
		searchString,
	} from "$common/stores";

	import LeftNav from "$lib/menu/LeftNav.svelte";
	import ResultCard from "$lib/search/SearchPrompt.svelte";

	import LeftButtonList from "./LeftButtonList.svelte";
	import LeftInfo from "./LeftInfo.svelte";
	import Links from "./Links.svelte";
	import RightButtonList from "./RightButtonList.svelte";
	import RightSearch from "./RightSearch.svelte";

	$: showResultCard = $forceAliveSearch || $searchString.length > 0;
</script>

<nav>
	<LeftButtonList />

	<LeftInfo />

	<spacer class="hide-mobile" style="height:2.25rem" />

	<Links />

	<RightSearch />

	<RightButtonList />

	{#if showResultCard}
		<ResultCard data={[]} />
	{/if}
</nav>

<style lang="scss">
	nav {
		display: flex;
		flex-direction: row;
		justify-content: start;
		align-items: center;
		border-bottom: 1px solid var(--border);
		background-color: var(--bg-1);
		height: 5rem;
		overflow: hidden;

		@media only screen and (max-width: 960px) {
			justify-content: space-between;
			border-color: var(--border-accent);
			padding: 0 2rem;
		}

		@media only screen and (max-width: 550px) {
			padding: 0 0.5rem;
		}

		spacer {
			border-right: 1px solid var(--border);
		}
	}
</style>
