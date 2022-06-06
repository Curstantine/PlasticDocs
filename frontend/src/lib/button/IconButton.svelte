<script lang="ts">
	export let viewBox: string = "0 0 24 24";
</script>

<button on:click>
	<svg xmlns="http://www.w3.org/2000/svg" {viewBox}>
		<slot />
	</svg>
</button>

<style lang="scss">
	button {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		height: 2.75rem;
		width: 2.75rem;
		border-radius: 5rem;
		color: var(--text-lighter);

		&:hover {
			background-color: var(--bg-2);
		}

		&:active {
			background-color: var(--bg-3);
		}

		svg {
			fill: currentColor;
			height: 24px;
			width: 24px;
		}
	}
</style>
