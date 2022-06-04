<script lang="ts">
	export let name: string;
</script>

<button>
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
