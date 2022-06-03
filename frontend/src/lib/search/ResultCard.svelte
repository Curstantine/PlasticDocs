<script lang="ts">
	import { fade } from "svelte/transition";
	import type { SearchData } from "$common/typings";
	import Spinner from "$lib/loaders/Spinner.svelte";

	export let data: Array<SearchData>;
	let showSpinner = !data || data.length === 0;
</script>

<wrapper transition:fade={{ duration: 150 }}>
	<card class:load={showSpinner}>
		{#if showSpinner}
			<Spinner height={48} width={48} />
		{/if}
	</card>
</wrapper>

<style lang="scss">
	wrapper {
		position: absolute;
		top: 5rem;
		left: 0;
		width: 100%;
		height: calc(100% - 5rem);
		display: flex;
		flex-direction: row;
		justify-content: end;

		@media only screen and (max-width: 960px) {
			background: rgba(0, 0, 0, 0.05);
		}

		card {
			z-index: 0;
			display: flex;
			flex-direction: col;
			height: 10rem;
			width: 32rem;
			background-color: var(--bg-1);
			border-radius: 0.25rem;
			margin-top: 0.5rem;
			margin-right: 0.5rem;
			box-shadow: 0px 1px 3px rgba(0, 0, 0, 0.1),
				0px 1px 4px rgba(0, 0, 0, 0.1), 0px 1px 6px rgba(0, 0, 0, 0.1);

			&.load {
				justify-content: center;
				align-items: center;
			}
		}
	}
</style>
