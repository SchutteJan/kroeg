<script lang="ts">
	import BarItem from '$lib/BarItem.svelte'
	import { user } from '$lib/stores'
	import { onMount } from 'svelte'
	import type { LocationResponse, UserRoleEnum } from '../models/schemas'
	import { get_bars } from '../api/bars'

	let bars: Array<LocationResponse> = []
	let loading = true
	let userRole: UserRoleEnum
	let only_published = true

	user.subscribe((value) => {
		if (value) {
			userRole = value.role
		}
	})

	function fetchBars() {
		loading = true
		get_bars(only_published)
			.then((response) => response.json())
			.then((data) => {
				bars = data
				loading = false
			})
	}

	onMount(async () => {
		fetchBars()
	})
</script>

<section>
	<h2>List</h2>
	<p>
		All {bars.length == 0 ? '' : bars.length} bars in Amsterdam listed.
		{#if userRole == 'Admin'}
			<label>
				<input
					type="checkbox"
					name="only_published"
					bind:checked={only_published}
					on:change={fetchBars}
				/>
				Show Only Published Bars
			</label>
		{/if}
	</p>

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
