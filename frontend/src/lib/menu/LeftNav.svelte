<script>
	import SelectItem from "./SelectItem.svelte";
	import SelectList from "./SelectList.svelte";
	import IconButton from "$lib/button/IconButton.svelte";

	import { forceAliveMenu } from "$common/stores";
	import { PAGES } from "$common/configuration";
	import FlexibleDrop from "$lib/extra/Backdrop.svelte";

	function handleCloseSearch() {
		$forceAliveMenu = false;
	}
</script>

{#if $forceAliveMenu}
	<FlexibleDrop on:click={() => ($forceAliveMenu = false)} />
{/if}

<nav class:mobile={$forceAliveMenu}>
	<mobile class="action-bar">
		<IconButton on:click={handleCloseSearch}>
			<path
				d="M13.41,12l6.3-6.29a1,1,0,1,0-1.42-1.42L12,10.59,5.71,4.29A1,1,0,0,0,4.29,5.71L10.59,12l-6.3,6.29a1,1,0,0,0,0,1.42,1,1,0,0,0,1.42,0L12,13.41l6.29,6.3a1,1,0,0,0,1.42,0,1,1,0,0,0,0-1.42Z"
			/>
		</IconButton>
	</mobile>

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
			top: 0;
			border-width: 0;
			position: absolute;
			display: flex;
			background-color: var(--bg-1);
			transform: translateX(-18rem);

			mobile.action-bar {
				display: flex;
				flex-direction: row;
			}

			&.mobile {
				transform: translateX(0);
			}
		}

		@media only screen and (max-width: 350px) {
			transform: translateX(-100%);
			width: 100%;
		}
	}
</style>
