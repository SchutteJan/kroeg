<script lang="ts">
	import BarItem from '../lib/BarItem.svelte';
	import { onMount } from 'svelte';
	import type { LocationResponse } from '../models/schemas';

	let bars: Array<LocationResponse> = [];
	let loading = true;

	onMount(async () => {
		const res = await fetch('http://localhost:8000/bars', {
			method: 'GET'
		});
		bars = await res.json();
		loading = false;
	});
</script>

<nav class="container-fluid">
	<ul>
		<li><a href="/">🍺</a></li>
	</ul>
</nav>

<main class="container">
	<section>
		<header class="container">
			<hgroup>
				<h1>Kroegen</h1>
				<p>Zie hier een lijst van alle kroegen in Amsterdam.</p>
			</hgroup>
		</header>

		<div class="bar-list-container">
			{#if loading}
				<progress />
			{:else}
				{#each bars as bar}
					<BarItem name={bar.name} description={bar.description ?? "No Description"} />
				{/each}
			{/if}
		</div>
	</section>
</main>
