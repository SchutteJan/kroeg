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

	interface AreaStat {
		total: number
		visited: number
	}

	function zipVistsByArea(total_bars: [string, number][], visited_bars: [string, number][]) {
		let areaMap = new Map<string, AreaStat>()
		total_bars.forEach(([name, total]) => {
			let visited = visited_bars.find(([n, _]) => n === name)?.[1] ?? 0
			areaMap.set(name, { total, visited: visited })
		})
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
					<th>Visits/Total</th>
				</tr>
			</thead>
			<tbody>
				{#each zipVistsByArea(visitStats.total_bars_by_area, visitStats.bar_visits_by_area).entries() as [name, stat]}
					<tr>
						<td>{name}</td>
						<td>{stat.visited}/{stat.total}</td>
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
