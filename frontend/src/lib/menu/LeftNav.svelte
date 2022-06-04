<script>
	import SelectItem from "./SelectItem.svelte";
	import SelectList from "./SelectList.svelte";

	import { PAGES } from "$common/configuration";
</script>

<nav class="top-nav">
	{#each PAGES as page}
		{#if page.nested_routes}
			<SelectList name={page.name} icon={page.item_icon_svg}>
				{#each page.nested_routes as nested_page}
					<SelectItem name={nested_page.name} path={nested_page.path}>
						{nested_page.item_icon_svg}
					</SelectItem>
				{/each}
			</SelectList>
		{:else}
			<SelectItem name={page.name} path={page.path}>
				{page.item_icon_svg}
			</SelectItem>
		{/if}
	{/each}
</nav>

<style lang="scss">
	nav {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 18rem;
		background-color: var(--bg-2);
		border-right: 1px solid var(--border);
		padding: 1rem;
		padding-right: 0;

		@media only screen and (max-width: 960px) {
			display: none;
		}
	}
</style>
