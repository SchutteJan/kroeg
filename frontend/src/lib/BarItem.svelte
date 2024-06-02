<script lang="ts">
	import type { LocationResponse } from '../models/schemas'

	export let bar: LocationResponse
	const placeholder =
		'https://images.jan.tf/ecmAqc89DiQEu0HlPMBcNxDFyigWMJI-xUJCNJAbklQ/fill/512/512/no/1/bG9jYWw6Ly8vYmFyLXBsYWNlaG9sZGVyLnBuZw.jpg'
</script>

<article class="bar-item">
	<div class="bar-image" style="background-image: url({bar.imageurl ?? placeholder})" />
	<div>
		<h2>{bar.name}</h2>
		<p>{bar.description ?? 'No Description'}</p>
		<small>x:{bar.coordinates.x}, y: {bar.coordinates.y} </small>
	</div>
</article>

<style>
	.bar-item {
		display: flex;
	}
	.bar-item > .bar-image {
		width: 10rem;
		height: 10rem;
		background-size: cover;
		background-position: center;
		margin: 0 var(--pico-block-spacing-horizontal);
		border-radius: var(--pico-border-radius);
	}
</style>
