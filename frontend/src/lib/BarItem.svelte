<script lang="ts">
	import type { LocationResponse, WhoResponse } from '../models/schemas'
	import { localDate } from '$lib/time'
	import { user } from '$lib/stores'
	import { deleteVisit, setPublished, visitBar } from '../api/bars'
	import Checkmark from './Checkmark.svelte'
	import Externallink from './Externallink.svelte'
	export let bar: LocationResponse
	export let isLoggedIn: boolean = false
	export let isAdmin: boolean = false

	user.subscribe((value: WhoResponse | undefined) => {
		isLoggedIn = value !== undefined
		if (value !== undefined) {
			isAdmin = value.role === 'Admin'
		}
	})
	const placeholder =
		'https://images.jan.tf/3AdpERPJX0kllHCcEVGrp6iPXEM0-Jili9_buKOsz24/rs:fit:128:128/plain/local:///bar-placeholder.png'

	function visitString() {
		if (bar.visited_at) {
			return localDate(bar.visited_at)
		}
		return null
	}

	function handleDeleteBarVisit() {
		if (confirm(`Are you sure you want to remove your visit to ${bar.name}?`)) {
			deleteVisit(bar.id).then(() => {
				bar.visited_at = null
			})
		}
	}

	function handleVisitBar() {
		visitBar(bar.id).then(() => {
			bar.visited_at = new Date().toISOString()
		})
	}

	function setSrcToPlaceholder(event: Event) {
		const img = event.target as HTMLImageElement
		if (img.src !== placeholder) {
			img.src = placeholder
		}
	}

	function toggleHideBar() {
		const hideStr = bar.published ? 'hide' : 'unhide'

		if (confirm(`Are you sure you want to ${hideStr} ${bar.name}?`)) {
			setPublished(bar.id, !bar.published).then(() => {
				bar.published = !bar.published
			})
		}
	}
</script>

<article>
	<details>
		<summary class="bar-item">
			<img
				loading="lazy"
				alt={bar.name}
				class="bar-image"
				src={bar.imageurl ?? placeholder}
				on:error={setSrcToPlaceholder}
			/>
			<div class="bar-content">
				{#if !bar.published}
					<h3><s>{bar.name}</s></h3>
				{:else}
					<h3>{bar.name}</h3>
				{/if}
				<p>
					{bar.address_line} •
					<span class="area">{bar.area_name ? bar.area_name : 'Unknown Area'}</span>
				</p>

				{#if isLoggedIn}
					{#if bar.visited_at}
						<span data-tooltip={visitString()} class="checkmark float"><Checkmark /></span>
					{:else}
						<button on:click={handleVisitBar} class="visit-button outline float">Check in</button>
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

		{#if isLoggedIn && isAdmin}
			<button class:pico-background-red={bar.published} on:click={toggleHideBar}>
				{#if bar.published}
					Hide
				{:else}
					Show
				{/if}
			</button>
		{/if}

		{#if isLoggedIn && bar.visited_at}
			<button class="visit-button visit-button-red outline" on:click={handleDeleteBarVisit}
				>Remove Visit</button
			>
		{/if}
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

	.float {
		float: right;
	}

	.visit-button {
		transition: transform 0.1s ease-in-out;
		border-radius: 999rem;
		padding: 0.5rem 0.75rem 0.5rem 0.75rem;
		color: var(--pico-primary);
		border-color: var(--pico-primary);
		margin-left: 0.3em;
	}

	.visit-button-red {
		color: var(--pico-color-red-400);
		border-color: var(--pico-color-red-400);
	}

	.visit-button:hover {
		transform: scale(1.05);
	}

	.checkmark {
		width: 1.5em;
		border-bottom: none;
	}

	.area {
		white-space: nowrap;
	}
</style>
