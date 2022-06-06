<script lang="ts">
	import { push } from "svelte-spa-router";

	import {
		DOC_VERSION,
		PRODUCT_ICON,
		PRODUCT_NAME,
	} from "$common/configuration";
</script>

<wrapper>
	<info on:click={() => push("/")}>
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

<style lang="scss">
	wrapper {
		min-width: 17rem;
		padding: 0 1rem;

		@media only screen and (max-width: 500px) {
			padding: 0;
			min-width: 0;
		}

		info {
			display: flex;
			flex-direction: row;
			align-items: center;
			height: 3rem;
			min-width: 16rem;
			justify-content: space-between;

			& > * {
				margin-right: 0.5rem;

				&:nth-last-child(1) {
					margin-right: 0;
				}
			}

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
	}
</style>
