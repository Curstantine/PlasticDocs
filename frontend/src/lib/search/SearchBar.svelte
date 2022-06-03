<script lang="ts">
	import { searchString, forceAliveSearch } from "$common/stores";
	export let height = "2rem";
	export let width = "16rem";
	export let mobileSupport = false;

	function handleChange() {
		if (mobileSupport) $forceAliveSearch = true;
		else $forceAliveSearch = false;
	} 
</script>

<search style="height: {height}; width: {width};">
	<input type="text" bind:value={$searchString} on:change={handleChange} placeholder="Search!" />
</search>

<style lang="scss">
	search {
		position: relative;
		display: flex;
		flex-direction: row;
		border-radius: 0.25rem;
		background-color: var(--bg-2);
		outline: 1px solid var(--border);
		transition: outline-color 150ms ease-out;

		&:focus-within {
			outline-color: var(--text-accent);
		}
	}

	input[type="text"] {
		position: absolute;
		width: inherit;
		height: inherit;
		left: 0;
		top: 0;
		padding: 0 1rem;
		background: none;
		color: var(--text-dark);
		font-weight: 600;
		&::placeholder {
			color: var(--text-lighter);
		}
	}
</style>
