<script lang="ts">
	import { fade } from "svelte/transition";
	import type { SearchData } from "$common/typings";
	import Spinner from "$lib/loaders/Spinner.svelte";

	export let data: Array<SearchData>;
	let showSpinner = !data || data.length === 0;
</script>

<wrapper transition:fade={{ duration: 150 }}>
	<card class:load={showSpinner}>
		<mobile class="action-bar">
			<button>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
					<path
						d="M13.41,12l6.3-6.29a1,1,0,1,0-1.42-1.42L12,10.59,5.71,4.29A1,1,0,0,0,4.29,5.71L10.59,12l-6.3,6.29a1,1,0,0,0,0,1.42,1,1,0,0,0,1.42,0L12,13.41l6.29,6.3a1,1,0,0,0,1.42,0,1,1,0,0,0,0-1.42Z"
					/>
				</svg>
			</button>
		</mobile>
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
			top: 0;
			height: 100%;
			background: rgba(0, 0, 0, 0.05);
			padding: 0.5rem 0;
		}

		card {
			display: flex;
			flex-direction: col;
			height: 10rem;
			width: 32rem;
			background-color: var(--bg-1);
			border-radius: 0.25rem;
			margin: 0.5rem 0.5rem;
			box-shadow: 0px 1px 3px rgba(0, 0, 0, 0.1),
				0px 1px 4px rgba(0, 0, 0, 0.1), 0px 1px 6px rgba(0, 0, 0, 0.1);

			@media only screen and (max-width: 960px) {
				height: calc(100% - 1rem);
			}

			&.load {
				justify-content: center;
				align-items: center;
			}
		}
	}
</style>
