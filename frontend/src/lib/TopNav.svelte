<script lang="ts">
	import ExtendSearch from "$lib/search/extend.svelte";
	import {
		PRODUCT_NAME,
		PRODUCT_ICON,
		DOC_VERSION,
		EXTERNAL_LINKS,
	} from "$common/constants";
</script>

<nav class="top-nav">
	<mobile>
		<button>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
				<path
					d="M12,10a2,2,0,1,0,2,2A2,2,0,0,0,12,10ZM5,10a2,2,0,1,0,2,2A2,2,0,0,0,5,10Zm14,0a2,2,0,1,0,2,2A2,2,0,0,0,19,10Z"
				/>
			</svg>
		</button>
	</mobile>

	<wrapper class="info">
		<info class="left">
			{#if PRODUCT_ICON.length > 0}
				<img src={PRODUCT_ICON} height="48" alt="logo" />
			{/if}

			<span class="name">{PRODUCT_NAME}</span>

			<container>
				<span class="version">
					{DOC_VERSION}
				</span>
			</container>
		</info>
	</wrapper>

	<spacer class="hide-mobile" style="height:2.25rem" />

	<links>
		{#each EXTERNAL_LINKS as item}
			<a class="item" href={item.url}>
				{#if item.icon && (typeof item.icon === "string" || !item.icon.isRawSvg)}
					<img
						src={typeof item.icon === "string"
							? item.icon
							: item.icon.value}
						height="24"
						alt={item.name}
					/>
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 22 22">
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
					</svg>
				{/if}

				<span>{item.name}</span>
			</a>
		{/each}
	</links>

	<wrapper class="info hide-mobile">
		<info class="right">
			<ExtendSearch />
		</info>
	</wrapper>

	<mobile>
		<button>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
				<path
					d="M21.71,20.29,18,16.61A9,9,0,1,0,16.61,18l3.68,3.68a1,1,0,0,0,1.42,0A1,1,0,0,0,21.71,20.29ZM11,18a7,7,0,1,1,7-7A7,7,0,0,1,11,18Z"
				/>
			</svg>
		</button>
	</mobile>
</nav>

<style lang="scss">
	.top-nav {
		display: flex;
		flex-direction: row;
		height: 5rem;
		overflow: hidden;
		background-color: var(--bg-1);
		border-bottom: 1px solid var(--border);
		align-items: center;
		justify-content: start;

		mobile {
			display: none;
		}

		.hide-mobile {
			@media only screen and (max-width: 960px) {
				display: none;
			}
		}

		@media only screen and (max-width: 960px) {
			padding: 0 2rem;
			justify-content: space-between;
			border-width: 2px;

			mobile {
				display: flex;

				button {
					display: flex;
					flex-direction: row;
					justify-content: center;
					align-items: center;
					height: 3rem;
					width: 3rem;
					border-radius: 5rem;
					color: var(--text-dark);
					transition: background-color 150ms ease-in;

					&:hover {
						background-color: var(--bg-2);
					}

					&:active {
						background-color: var(--bg-3);
					}

					svg {
						fill: currentColor;
						height: 30px;
						width: 30px;
					}
				}
			}
		}

		spacer {
			border-right: 1px solid var(--border);
		}

		wrapper.info {
			min-width: 17rem;
			padding: 0 1rem;
		}

		info {
			display: flex;
			flex-direction: row;
			align-items: center;
			justify-content: center;

			height: 3rem;
		}

		info.left {
			* {
				margin-right: 0.5rem;
			}

			:nth-last-child(1) {
				margin-right: 0;
			}

			span {
				user-select: none;

				&.name {
					font-size: x-large;
					font-weight: 600;
				}

				&.version {
					font-size: small;
					font-weight: 600;
					color: var(--text-accent);
				}
			}

			container {
				display: flex;
				flex-direction: row;
				justify-content: center;
				align-items: center;
				padding: 0.25rem;
				border-radius: 0.25rem;

				background-color: var(--bg-accent);
			}
		}

		info.right {
			justify-content: end;
		}

		links {
			display: flex;
			flex-direction: row;
			margin-left: 4rem;
			height: 2.5rem;
			flex: 1;

			@media only screen and (max-width: 960px) {
				display: none;
			}

			& > * {
				margin-right: 1rem;
			}

			:nth-last-child(1) {
				margin-right: 0;
			}

			a.item {
				display: flex;
				flex-direction: row;
				align-items: center;
				justify-content: start;
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
					height: 20px;
					width: 20px;
					fill: currentColor;
				}

				span {
					font-size: medium;
					font-weight: 500;
					transition: all 150ms ease-in;
				}
			}
		}
	}
</style>
