<script lang="ts">
	import { fade } from "svelte/transition";
</script>

<backdrop transition:fade={{ duration: 150 }}>
	<slot />
</backdrop>

<style lang="scss">
	backdrop {
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
			background: rgba(0, 0, 0, 0.1);
		}
	}
</style>
