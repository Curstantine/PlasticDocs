<script>
	import SelectItem from "./SelectItem.svelte";
	import SelectList from "./SelectList.svelte";
</script>

<nav class="top-nav">
	<SelectItem name="Home">
		<path
			d="M20,8h0L14,2.74a3,3,0,0,0-4,0L4,8a3,3,0,0,0-1,2.26V19a3,3,0,0,0,3,3H18a3,3,0,0,0,3-3V10.25A3,3,0,0,0,20,8ZM14,20H10V15a1,1,0,0,1,1-1h2a1,1,0,0,1,1,1Zm5-1a1,1,0,0,1-1,1H16V15a3,3,0,0,0-3-3H11a3,3,0,0,0-3,3v5H6a1,1,0,0,1-1-1V10.25a1,1,0,0,1,.34-.75l6-5.25a1,1,0,0,1,1.32,0l6,5.25a1,1,0,0,1,.34.75Z"
		/>
	</SelectItem>
	<SelectItem name="Docs" />

	<SelectList name="Guides">
		<SelectItem name="Getting Started but the line should be overflowing">
			<path
				d="M20,8h0L14,2.74a3,3,0,0,0-4,0L4,8a3,3,0,0,0-1,2.26V19a3,3,0,0,0,3,3H18a3,3,0,0,0,3-3V10.25A3,3,0,0,0,20,8ZM14,20H10V15a1,1,0,0,1,1-1h2a1,1,0,0,1,1,1Zm5-1a1,1,0,0,1-1,1H16V15a3,3,0,0,0-3-3H11a3,3,0,0,0-3,3v5H6a1,1,0,0,1-1-1V10.25a1,1,0,0,1,.34-.75l6-5.25a1,1,0,0,1,1.32,0l6,5.25a1,1,0,0,1,.34.75Z"
			/>
		</SelectItem>
		<SelectItem name="Configuration" />
	</SelectList>
</nav>

<style lang="scss">
	nav {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 18rem;
		background-color: var(--bg-2);
		border-right: 1px solid var(--border);
		padding: 1rem;
		padding-right: 0;

		@media only screen and (max-width: 960px) {
			display: none;
		}
	}
</style>
