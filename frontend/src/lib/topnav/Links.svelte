<script lang="ts">
	import { EXTERNAL_LINKS } from "$common/configuration";
</script>

<links>
	{#each EXTERNAL_LINKS as item}
		<a href={item.url}>
			{#if item.icon || !item.icon.isRawSvg}
				<img
					src={typeof item.icon === "string"
						? item.icon
						: item.icon.value}
					height="24"
					alt={item.name}
				/>
			{:else}
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 22 22">
					{#if item.icon && item.icon.isRawSvg}
						{item.icon.value}
					{:else}
						<g>
							<g>
								<rect width="20" height="20" opacity="0" />
								<path
									d="M20 11a1 1 0 0 0-1 1v6a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h6a1 1 0 0 0 0-2H6a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3h12a3 3 0 0 0 3-3v-6a1 1 0 0 0-1-1z"
								/>
								<path
									d="M16 5h1.58l-6.29 6.28a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0L19 6.42V8a1 1 0 0 0 1 1 1 1 0 0 0 1-1V4a1 1 0 0 0-1-1h-4a1 1 0 0 0 0 2z"
								/>
							</g>
						</g>
					{/if}
				</svg>
			{/if}

			<span>{item.name}</span>
		</a>
	{/each}
</links>

<style lang="scss">
	links {
		flex: 1;
		display: flex;
		flex-direction: row;
		margin-left: 4rem;
		height: 2.5rem;

		@media only screen and (max-width: 960px) {
			display: none;
		}

		& > * {
			margin-right: 1rem;
		}

		:nth-last-child(1) {
			margin-right: 0;
		}

		a {
			display: flex;
			flex-direction: row;
			justify-content: start;
			align-items: center;
			color: var(--text-light);

			&:hover {
				color: var(--text-lighter);
			}

			& > * {
				margin-right: 0.5rem;
			}

			:nth-last-child(1) {
				margin-right: 0;
			}

			svg {
				fill: currentColor;
				width: 20px;
				height: 20px;
			}

			span {
				font-weight: 500;
				font-size: medium;
			}
		}
	}
</style>
