<script lang="ts">
	export let viewBox: string = "0 0 24 24";
</script>

<button>
	<svg xmlns="http://www.w3.org/2000/svg" {viewBox}>
		<slot />
	</svg>
</button>

<style lang="scss">
	:global(mobile) > button {
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
</style>
