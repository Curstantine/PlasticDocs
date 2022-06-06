<script lang="ts">
	import { sineInOut } from "svelte/easing";
	import { slide } from "svelte/transition";

	export let name: string;
	export let icon = "";
	$: collapased = false;
</script>

<list class:collapased>
	<row class="info" on:click={() => (collapased = !collapased)}>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
			{icon}
		</svg>

		<span>{name}</span>

		<button class="arrow">
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
				<path
					d="M17,9.17a1,1,0,0,0-1.41,0L12,12.71,8.46,9.17a1,1,0,0,0-1.41,0,1,1,0,0,0,0,1.42l4.24,4.24a1,1,0,0,0,1.42,0L17,10.59A1,1,0,0,0,17,9.17Z"
				/>
			</svg>
		</button>
	</row>

	{#if !collapased}
		<group
			class:collapased
			transition:slide={{ duration: 150, easing: sineInOut }}
		>
			<slot />
		</group>
	{/if}
</list>

<style lang="scss">
	@use "./common.scss";

	list {
		display: flex;
		flex-direction: column;
		width: 100%;

		&.collapased row.info button.arrow svg {
			transform: rotate(-90deg);
		}

		row {
			@include common.createListItem;

			button.arrow {
				justify-items: end;
				display: flex;
				flex-direction: row;
				justify-content: end;
				align-items: center;
				justify-content: center;
				border-radius: 5rem;
				margin-right: 0.5rem;

				width: 2rem;
				height: 2rem;

				svg {
					height: 22px;
					width: 22px;
					fill: var(--text-light);
				}

				&:hover {
					background-color: var(--bg-3);

					svg {
						fill: var(--text-accent-lighter);
					}
				}
			}
		}

		group {
			display: flex;
			flex-direction: column;
			height: fit-content;
			padding-left: 1.5rem;

			&.collapased {
				display: none;
				transition: display 50ms ease-in-out;
			}
		}
	}
</style>
