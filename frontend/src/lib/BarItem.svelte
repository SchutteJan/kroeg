<script lang="ts">
	import type { LocationResponse } from '../models/schemas'
	import { localDate } from '$lib/time'
	import { user } from '$lib/stores'
	import { visitBar } from '../api/bars'
	import Checkmark from './Checkmark.svelte'
	export let bar: LocationResponse
	export let isLoggedIn: boolean = false

	user.subscribe((value) => (isLoggedIn = value !== undefined))
	const placeholder =
		'https://images.jan.tf/ecmAqc89DiQEu0HlPMBcNxDFyigWMJI-xUJCNJAbklQ/fill/512/512/no/1/bG9jYWw6Ly8vYmFyLXBsYWNlaG9sZGVyLnBuZw.jpg'

	function handleVisitBar() {
		if (bar.visited_at) {
			// TODO: Remove visit
			return
		}
		visitBar(bar.id).then(() => {
			bar.visited_at = new Date().toISOString()
		})
	}
</script>

<article class="bar-item">
	<img alt={bar.name} class="bar-image" src={bar.imageurl ?? placeholder} />
	<div class="bar-content">
		<h3>{bar.name}</h3>
		<p>{bar.description ?? 'No Description'}</p>
		{#if isLoggedIn}
			<p>
				<button
					on:click={() => handleVisitBar()}
					class="visit-button outline"
					class:visited={bar.visited_at}
				>
					{#if bar.visited_at}
						<span class="checkmark"><Checkmark /></span> {localDate(bar.visited_at)}
					{:else}
						Mark Visited
					{/if}
				</button>
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

	.bar-content {
		flex-grow: 1;
	}

	.visit-button {
		border-radius: 999rem;
		padding: 0.5rem 0.75rem 0.5rem 0.75rem;
		float: right;
		color: var(--pico-color-zinc-200);
		border-color: var(--pico-color-zinc-200);
	}

	.visit-button.visited {
		color: var(--pico-color-green-400);
		border-color: var(--pico-color-green-400);
		cursor: auto;
	}

	.checkmark {
		width: 1em;
		display: inline-block;
		vertical-align: middle;
		margin-right: 2px;
		margin-bottom: 0.1em;
	}
</style>
