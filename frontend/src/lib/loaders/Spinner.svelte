<script lang="ts">
	import { afterUpdate, onMount } from "svelte";
	import { fade } from "svelte/transition";

	export let height: number = 32;
	export let width: number = 32;

	let circle: any;
	let value = undefined;
	function updateValue() {
		const circumference = Math.PI * (circle.getAttribute("r") * 2);
		if (value < 0) value = 0;
		if (value > 100) value = 100;
		circle.style.strokeDashoffset = ((100 - value) / 100) * circumference;
	}

	onMount(updateValue);
	afterUpdate(updateValue);
</script>

<svg
	class="spinner"
	style="height: {height}; width: {width};"
	{width}
	{height}
	viewBox="0 0 16 16"
	role="progressbar"
	in:fade={{ delay: 400, duration: 150 }}
>
	<circle class="spinner-ring" cx="50%" cy="50%" r="7" />
	<circle
		class="spinner-fill"
		cx="50%"
		cy="50%"
		r="7"
		stroke-dasharray="3"
		bind:this={circle}
	/>
</svg>

<style lang="scss">
	@keyframes spinner-indeterminate {
		0% {
			stroke-dasharray: 0.01px 43.97px;
			transform: rotate(0deg);
		}

		50% {
			transform: rotate(450deg);
			stroke-dasharray: 21.99px 21.99px;
		}

		100% {
			stroke-dasharray: 0.01px 43.97px;
			transform: rotate(1080deg);
		}
	}

	.spinner circle {
		fill: none;
		transform: rotate(-90deg);
		transition: all 0.2s ease-in-out;
		stroke-width: 2;
		stroke-linecap: round;
		transform-origin: 50% 50%;
	}

	.spinner-ring {
		stroke: var(--bg-3);
	}

	.spinner .spinner-fill {
		stroke: var(--border-accent);
		stroke-dasharray: 43.75;
		animation: spinner-indeterminate 2s linear infinite;
	}
</style>
