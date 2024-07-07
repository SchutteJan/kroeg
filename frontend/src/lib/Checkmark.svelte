<script lang="ts">
	// https://codepen.io/elevaunt/pen/JYRBzJ
	export let success: boolean = true
</script>

{#if success}
	<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 130.2 130.2">
		<circle
			class="path circle success"
			fill="none"
			stroke-width="10"
			stroke-miterlimit="10"
			cx="65.1"
			cy="65.1"
			r="62.1"
		/>
		<polyline
			class="path check success"
			fill="none"
			stroke-width="10"
			stroke-linecap="round"
			stroke-miterlimit="10"
			points="100.2,40.2 51.5,88.8 29.8,67.5 "
		/>
	</svg>
{:else}
	<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 130.2 130.2">
		<circle
			class="path circle error"
			fill="none"
			stroke-width="10"
			stroke-miterlimit="10"
			cx="65.1"
			cy="65.1"
			r="62.1"
		/>
		<line
			class="path line error"
			fill="none"
			stroke-width="10"
			stroke-linecap="round"
			stroke-miterlimit="10"
			x1="34.4"
			y1="37.9"
			x2="95.8"
			y2="92.3"
		/>
		<line
			class="path line error"
			fill="none"
			stroke-width="10"
			stroke-linecap="round"
			stroke-miterlimit="10"
			x1="95.8"
			y1="38"
			x2="34.4"
			y2="92.2"
		/>
	</svg>
{/if}

<style>
	svg {
		display: block;
		overflow: visible;
	}
	.path {
		stroke-dasharray: 1000;
		stroke-dashoffset: 0;
	}
	.success {
		stroke: var(--pico-color-green-400);
	}
	.error {
		stroke: var(--pico-color-red-400);
	}
	.path.circle {
		animation: dash 0.9s ease-in-out;
	}
	.path.line {
		stroke-dashoffset: 1000;
		animation: dash 0.9s 0.35s ease-in-out forwards;
	}
	.path.check {
		stroke-dashoffset: -100;
		animation: dash-check 0.9s 0.35s ease-in-out forwards;
	}

	@keyframes dash {
		0% {
			stroke-dashoffset: 1000;
		}
		100% {
			stroke-dashoffset: 0;
		}
	}
	@keyframes dash-check {
		0% {
			stroke-dashoffset: -100;
		}
		100% {
			stroke-dashoffset: 900;
		}
	}
</style>
