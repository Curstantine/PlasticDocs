<script lang="ts">
	const productIcon = "/svelte.png";
	const productName = "PlasticDocs";
	const version = "0.0.1-dev";
	let externalLinks: ExtLink[] = [
		{
			name: "GitHub",
			url: "https://github.com/Curstantine/PlasticDocs",
			icon: { value: "/GitHub-Mark-32px.png" },
		},
		{
			name: "Discord",
			url: "https://discord.com/invite/discord",
		},
	];

	type ExtLink = {
		name: string;
		url: string;
		icon?: { isRawSvg?: boolean; value: string } | string;
	};
</script>

<nav class="top-nav">
	<wrapper class="info">
		<info>
			{#if productIcon.length > 0}
				<img src={productIcon} height="48" alt="logo" />
			{/if}

			<span class="name">{productName}</span>

			<container>
				<span class="version">
					{version}
				</span>
			</container>
		</info>
	</wrapper>

	<spacer style="height:2.25rem" />

	<links>
		{#each externalLinks as item}
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
</nav>

<style lang="scss">
	.top-nav {
		display: flex;
		flex-direction: row;
		height: 5rem;
		background-color: var(--bg-1);
		border-bottom: 1px solid var(--border);
		align-items: center;

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
			width: 100%;

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

		links {
			display: flex;
			flex-direction: row;
			margin-left: 4rem;
			height: 2.5rem;

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
