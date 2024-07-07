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
	<img alt={bar.name} class="bar-image" src={bar.imageurl ?? placeholder} />
	<div class="bar-content">
		<h3>{bar.name}</h3>
		<p>{bar.description ?? 'No Description'}</p>
		{#if bar.visited_at && isLoggedIn}
			<p>Visited on: {localDate(bar.visited_at)}</p>
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
		transition: transform 0.1s ease-in-out;
	}

	.bar-item:hover {
		transform: scale(1.01);
	}

	.bar-image {
		width: 10rem;
		height: 10rem;
		margin: 0 var(--pico-block-spacing-horizontal);
		border-radius: var(--pico-border-radius);
		max-width: 100%;
		object-fit: cover;
		align-self: center;
	}
</style>
