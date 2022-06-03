<script lang="ts">
	import { searchString } from "$common/stores";
</script>

<search>
	<input type="text" bind:value={$searchString} placeholder="Search!" />
</search>

<style lang="scss">
	search {
		position: relative;
		display: flex;
		flex-direction: row;
		height: 2rem;
		width: 16rem;
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
		width: 16rem;
		height: 2rem;
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
