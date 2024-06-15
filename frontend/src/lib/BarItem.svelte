<script lang="ts">
	import type { LocationResponse } from '../models/schemas'
	import { localDate } from '$lib/time'
	import { user } from '$lib/stores'
	import { visitBar } from '../api/bars'
	export let bar: LocationResponse
	export let isLoggedIn: boolean = false

	user.subscribe((value) => (isLoggedIn = value !== undefined))
	const placeholder =
		'https://images.jan.tf/ecmAqc89DiQEu0HlPMBcNxDFyigWMJI-xUJCNJAbklQ/fill/512/512/no/1/bG9jYWw6Ly8vYmFyLXBsYWNlaG9sZGVyLnBuZw.jpg'

	function handleVisitBar(id: number) {
		visitBar(id).then(() => {
			bar.visited_at = new Date().toISOString()
		})
	}
</script>

<article class="bar-item">
	<div class="bar-image" style="background-image: url({bar.imageurl ?? placeholder})" />
	<div>
		<h2>{bar.name}</h2>
		<p>{bar.description ?? 'No Description'}</p>
		<p>
			<small>x:{bar.coordinates.x}, y: {bar.coordinates.y} </small>
		</p>
		{#if bar.visited_at && isLoggedIn}
			<p>First visited on: {localDate(bar.visited_at)}</p>
		{:else if isLoggedIn}
			<p>
				<button on:click={() => handleVisitBar(bar.id)} class="outline">Mark Visited</button>
			</p>
		{/if}
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
