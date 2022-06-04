<script lang="ts">
	import { push } from "svelte-spa-router";

	export let name: string;
	export let path: string;
</script>

<button on:click={() => push(path)}>
	<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
		<slot />
	</svg>
	<span>{name}</span>
</button>

<style lang="scss">
	@use "./common.scss";

	button {
		@include common.createListItem(true);
	}
</style>
