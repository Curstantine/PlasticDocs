<script lang="ts">
	import SearchBar from "$lib/search/SearchBar.svelte";
</script>

<wrapper class="info hide-mobile">
	<info class="right">
		<SearchBar />
	</info>
</wrapper>

<style lang="scss">
	info {
		height: 3rem;
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: end;
	}
</style>
