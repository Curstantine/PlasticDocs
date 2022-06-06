<script lang="ts">
	import IconButton from "$lib/button/IconButton.svelte";
</script>

<mobile>
	<wrapper>
		<IconButton>
			<path
				d="M12,10a2,2,0,1,0,2,2A2,2,0,0,0,12,10ZM5,10a2,2,0,1,0,2,2A2,2,0,0,0,5,10Zm14,0a2,2,0,1,0,2,2A2,2,0,0,0,19,10Z"
			/>
		</IconButton>
	</wrapper>
</mobile>

<style lang="scss">
	wrapper {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		margin-right: 1rem;
		width: max-content;

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
</style>
