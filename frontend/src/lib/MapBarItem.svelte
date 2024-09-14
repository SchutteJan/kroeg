<script lang="ts">
	import type { LocationResponse, WhoResponse } from '../models/schemas'
	import { user } from '$lib/stores'
	import Checkmark from './Checkmark.svelte'
	import { visitBar, deleteVisit } from '../api/bars'
	import Externallink from './Externallink.svelte'

	export let bar: LocationResponse
	export let onVisitCallback: CallableFunction
	export let onDeleteVisitCallback: CallableFunction

	let isLoggedIn: boolean = false

	user.subscribe((value: WhoResponse | undefined) => {
		isLoggedIn = value !== undefined
	})

	function handleDeleteBarVisit() {
		if (confirm(`Are you sure you want to remove your visit to ${bar.name}?`)) {
			deleteVisit(bar.id).then(() => {
				bar.visited_at = null
				onDeleteVisitCallback()
			})
		}
	}

	function handleVisitBar() {
		visitBar(bar.id).then(() => {
			bar.visited_at = new Date().toISOString()
			onVisitCallback()
		})
	}
</script>

<div class="content">
	<h4 class="title">{bar.name}</h4>
	<p class="area">{bar.area_name}</p>

	<div>
		<a
			class="maps-link"
			target="_blank"
			href="https://www.google.com/maps/search/?api=1&query={encodeURIComponent(
				bar.address_line
			)}&query_place_id={bar.google_place_id}"
			>Maps <Externallink />
		</a>

		{#if isLoggedIn}
			{#if bar.visited_at}
				<button on:click={handleDeleteBarVisit} class="hidden-button"
					><span class="checkmark"><Checkmark /></span></button
				>
			{:else}
				<button on:click={handleVisitBar} class="visit-button outline">Check in</button>
			{/if}
		{/if}
	</div>
</div>

<style>
	.title,
	.area {
		/* Override the color to make it consistent between dark and light mode */
		color: var(--pico-color-zinc-700);
	}
	.checkmark {
		display: inline-block;
		width: 2em;
	}
	.content {
		text-align: center;
	}
	.hidden-button {
		all: unset;
		display: inline-block;
		cursor: pointer;
		vertical-align: middle;
	}
	.maps-link {
		font-size: 12pt;
	}
</style>
