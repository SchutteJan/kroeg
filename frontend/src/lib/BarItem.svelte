<script lang="ts">
	import type { LocationResponse } from '../models/schemas'
	import { localDate } from '$lib/time'
	import { user } from '$lib/stores'
	import { visitBar } from '../api/bars'
	import Checkmark from './Checkmark.svelte'
	import Externallink from './Externallink.svelte'
	export let bar: LocationResponse
	export let isLoggedIn: boolean = false

	user.subscribe((value) => (isLoggedIn = value !== undefined))
	const placeholder =
		'https://images.jan.tf/ecmAqc89DiQEu0HlPMBcNxDFyigWMJI-xUJCNJAbklQ/fill/512/512/no/1/bG9jYWw6Ly8vYmFyLXBsYWNlaG9sZGVyLnBuZw.jpg'

	function visitString() {
		if (bar.visited_at) {
			return localDate(bar.visited_at)
		}
		return null
	}

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

<article>
	<details>
		<summary class="bar-item">
			<img alt={bar.name} class="bar-image" src={bar.imageurl ?? placeholder} />
			<div class="bar-content">
				<h3>{bar.name}</h3>

				<p>
					{bar.address_line} •
					<span class="area">{bar.area_name ? bar.area_name : 'Unknown Area'}</span>
				</p>

				{#if isLoggedIn}
					{#if bar.visited_at}
						<span data-tooltip={visitString()} class="checkmark"><Checkmark /></span>
					{:else}
						<button on:click={handleVisitBar} class="visit-button outline">Check in</button>
					{/if}
				{/if}
			</div>
		</summary>
		<hr />
		<p>
			{#if bar.description}
				{bar.description}
			{:else}
				<i>No description available.</i>
			{/if}
		</p>
		<a
			target="_blank"
			href="https://www.google.com/maps/search/?api=1&query={encodeURIComponent(
				bar.address_line
			)}&query_place_id={bar.google_place_id}"
			>Open in Maps <Externallink />
		</a>
	</details>
</article>

<style>
	.bar-item {
		display: flex;
	}

	.bar-image {
		width: 6rem;
		height: 6rem;
		border-radius: var(--pico-border-radius);
		max-width: 100%;
		object-fit: cover;
		align-self: center;
	}

	.bar-content {
		flex-grow: 1;
		margin-left: var(--pico-block-spacing-horizontal);
	}

	.bar-content > h3 {
		word-break: break-word;
	}

	.visit-button {
		transition: transform 0.1s ease-in-out;
		border-radius: 999rem;
		padding: 0.5rem 0.75rem 0.5rem 0.75rem;
		float: right;
		color: var(--pico-primary);
		border-color: var(--pico-primary);
	}

	.visit-button:hover {
		transform: scale(1.05);
	}

	.checkmark {
		float: right;
		width: 1.5em;
		border-bottom: none;
	}

	.area {
		white-space: nowrap;
	}
</style>
