<script lang="ts">
	import type { LocationResponse, WhoResponse } from '../models/schemas'
	import { user } from '$lib/stores'
	import Checkmark from './Checkmark.svelte'
	import { visitBar } from '../api/bars'

	export let bar: LocationResponse
	export let onVisitCallback: CallableFunction

	let isLoggedIn: boolean = false

	user.subscribe((value: WhoResponse | undefined) => {
		isLoggedIn = value !== undefined
	})

	function handleVisitBar() {
		if (bar.visited_at) {
			// TODO: Remove visit
			return
		}
		visitBar(bar.id).then(() => {
			bar.visited_at = new Date().toISOString()
			onVisitCallback()
		})
	}
</script>

<div class="content">
	<h4 class="title">{bar.name}</h4>
	<p class="area">{bar.area_name}</p>

	{#if isLoggedIn}
		<div>
			{#if bar.visited_at}
				<span class="checkmark"><Checkmark /></span>
			{:else}
				<button on:click={handleVisitBar} class="visit-button outline">Check in</button>
			{/if}
		</div>
	{/if}
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
</style>
