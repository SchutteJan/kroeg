<script lang="ts">
	import { user } from '$lib/stores'
	import { onMount } from 'svelte'
	import type { VisitStats, WhoResponse } from '../../models/schemas'
	import { get_bar_visit_stats, logout } from '../../api/session'

	export let userData: WhoResponse | undefined = undefined
	user.subscribe((value) => (userData = value))

	export let visitStats: VisitStats | undefined = undefined

	async function handleLogout() {
		logout().then(() => {
			user.set(undefined)
		})
	}

	function zipVistsByArea(total_bars: [string, number][], visit_bars: [string, number][]) {
		let areaMap = new Map<string, [number, number]>()
		// zip two array together using map: a.map((k, i) => [k, b[i]])
		// this assumes that the two arrays are sorted in the same order
		total_bars.map((k, i) => areaMap.set(k[0], [k[1], visit_bars[i][1]]))
		return areaMap
	}

	onMount(async () => {
		get_bar_visit_stats()
			.then((response) => {
				response.json().then((data) => {
					visitStats = data
				})
			})
			// TODO: Handle these errors properly
			.catch((error) => {
				console.error('Failed to get visit stats', error)
			})
	})
</script>

<section>
	<h2>Me</h2>
	{#if userData}
		<p>
			This is you.
			{#if userData.role !== 'User'}
				You are logged in with role '{userData.role}'
			{/if}
		</p>
	{:else}
		<p>This could've been you.</p>
		<p>Uhuh not sure who you are, are you even <a href="/login">logged in</a>?</p>
	{/if}
	<h3>Statistics</h3>
	{#if visitStats && userData}
		Total bars visited: {visitStats.distinct_bar_visits}

		<table class="striped">
			<thead>
				<tr>
					<th>Area</th>
					<th>Total</th>
					<th>Visits</th>
				</tr>
			</thead>
			<tbody>
				{#each zipVistsByArea(visitStats.total_bars_by_area, visitStats.bar_visits_by_area).entries() as [name, [total, count]]}
					<tr>
						<td>{name}</td>
						<td>{total}</td>
						<td>{count}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{:else if userData}
		<p>Fetching visit stats...</p>
		<progress />
	{/if}
	<h3>Management</h3>
	{#if userData}
		<button on:click={handleLogout} class="outline">Logout</button>
	{/if}
</section>
