<script lang="ts">
	import BarItem from '../lib/BarItem.svelte'
	import { onMount } from 'svelte'
	import type { LocationResponse } from '../models/schemas'
	import { get_bars } from '../api/bars'

	let bars: Array<LocationResponse> = []
	let loading = true

	onMount(async () => {
		get_bars()
			.then((response) => response.json())
			.then((data) => {
				bars = data
				loading = false
			})
	})
</script>

<section>
	<h2>List</h2>
	<p>All {bars.length == 0 ? '' : bars.length} bars in Amsterdam listed.</p>

	<div class="bar-list-container">
		{#if loading}
			<progress />
		{:else}
			{#each bars as bar}
				<BarItem {bar} />
			{/each}
		{/if}
	</div>
</section>
