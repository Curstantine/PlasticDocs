<script lang="ts">
	import { getContext } from "svelte";
	import { push } from "svelte-spa-router";

	import {
		DOC_VERSION,
		EXTERNAL_LINKS,
		PRODUCT_ICON,
		PRODUCT_NAME,
	} from "$common/configuration";
	import { forceAliveSearch, searchString } from "$common/stores";
	import { Theme, ThemeKey } from "$common/utils";

	import IconButton from "$lib/button/IconButton.svelte";
	import SearchBar from "$lib/search/SearchBar.svelte";
	import ResultCard from "$lib/search/SearchPrompt.svelte";

	const themeHandler = getContext<Theme>(ThemeKey);
	$: showResultCard = $forceAliveSearch || $searchString.length > 0;
</script>

<nav>
	<mobile>
		<IconButton>
			<path
				d="M12,10a2,2,0,1,0,2,2A2,2,0,0,0,12,10ZM5,10a2,2,0,1,0,2,2A2,2,0,0,0,5,10Zm14,0a2,2,0,1,0,2,2A2,2,0,0,0,19,10Z"
			/>
		</IconButton>
	</mobile>

	<wrapper class="info">
		<info class="left" on:click={() => push("/")}>
			{#if PRODUCT_ICON.length > 0}
				<img src={PRODUCT_ICON} height="45" alt="logo" />
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
			<SearchBar />
		</info>
	</wrapper>

	<wrapper class="button-list">
		<mobile>
			<IconButton on:click={() => ($forceAliveSearch = true)}>
				<path
					d="M21.71,20.29,18,16.61A9,9,0,1,0,16.61,18l3.68,3.68a1,1,0,0,0,1.42,0A1,1,0,0,0,21.71,20.29ZM11,18a7,7,0,1,1,7-7A7,7,0,0,1,11,18Z"
				/>
			</IconButton>
		</mobile>

		<IconButton on:click={() => themeHandler.invert()}>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
				<path
					d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
				/>
			</svg>
		</IconButton>
	</wrapper>

	{#if showResultCard}
		<ResultCard data={[]} />
	{/if}
</nav>

<style lang="scss">
	nav {
		z-index: 10;
		display: flex;
		flex-direction: row;
		height: 5rem;
		overflow: hidden;
		background-color: var(--bg-1);
		border-bottom: 1px solid var(--border);
		align-items: center;
		justify-content: start;

		@media only screen and (max-width: 960px) {
			padding: 0 2rem;
			border-color: var(--border-accent);
			justify-content: space-between;
		}

		@media only screen and (max-width: 550px) {
			padding: 0 0.5rem;
		}

		spacer {
			border-right: 1px solid var(--border);
		}

		wrapper.info {
			min-width: 17rem;
			padding: 0 1rem;

			@media only screen and (max-width: 500px) {
				padding: 0;
				min-width: 0;
			}
		}

		wrapper.button-list {
			display: flex;
			flex-direction: row;
			align-items: center;
			justify-content: space-between;
			width: max-content;
			margin-right: 1rem;

			@media only screen and (max-width: 960px) {
				margin-right: 0;

				& > * {
					margin-right: 0.5rem;
				}
			}

			@media only screen and (max-width: 450px) {
				& > * {
					margin-right: 0.25rem;
				}
			}
		}

		info {
			display: flex;
			flex-direction: row;
			align-items: center;
			height: 3rem;

			& > * {
				margin-right: 0.5rem;

				&:nth-last-child(1) {
					margin-right: 0;
				}
			}
		}

		info.left {
			min-width: 16rem;
			justify-content: space-between;

			@media only screen and (max-width: 450px) {
				justify-content: center;
				min-width: max-content;
			}

			span {
				user-select: none;

				@media only screen and (max-width: 350px) {
					display: none;
				}

				&.name {
					font-size: x-large;
					font-weight: 700;
					color: var(--text-dark);
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

				@media only screen and (max-width: 450px) {
					display: none;
				}
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
