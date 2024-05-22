<script lang="ts">
	import BarItem from '../lib/BarItem.svelte';
	import { onMount } from 'svelte';
	import type { LocationResponse } from '../models/schemas';

	let bars: Array<LocationResponse> = [];
	let loading = true;

	onMount(async () => {
		const res = await fetch('/bars', {
			method: 'GET'
		});
		bars = await res.json();
		loading = false;
	});
</script>

<section>
	<h2>List</h2>
	<p>All kroegen listed.</p>

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
