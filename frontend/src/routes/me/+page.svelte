<script lang="ts">
	import { user } from '$lib/stores'
	import { onMount } from 'svelte'
	import type { VisitStats, WhoResponse, LocationResponse } from '../../models/schemas'
	import { get_bar_visit_stats, logout } from '../../api/session'
	import { get_bars } from '../../api/bars'
	import { localDate } from '$lib/time'
	import { toGmapsUrl } from '$lib/gmaps'

	export let userData: WhoResponse | undefined = undefined
	user.subscribe((value) => (userData = value))

	export let visitStats: VisitStats | undefined = undefined
	export let recentVisits: LocationResponse[] = []
	export let recentVisitOverflow = 0

	const VISIBLE_RECENT_VISITS = 25

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
		// TODO: don't re-fetch bars
		get_bars()
			.then((response) => {
				response.json().then((data: LocationResponse[]) => {
					const recent = data
						.filter((bar) => bar.visited_at !== null)
						.sort(
							(a, b) =>
								new Date(b.visited_at ?? ' ').getTime() - new Date(a.visited_at ?? ' ').getTime()
						)
					recentVisitOverflow = Math.max(0, recent.length - VISIBLE_RECENT_VISITS)
					recentVisits = recent.slice(0, VISIBLE_RECENT_VISITS)
				})
			})
			.catch((error) => {
				console.error('Failed to get recent visits', error)
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

	<h3>Recent Bar Visits</h3>
	{#if recentVisits.length > 0}
		<table class="striped">
			<thead>
				<tr>
					<th>Bar Name</th>
					<th>Area</th>
					<th>Visited On</th>
				</tr>
			</thead>
			<tbody>
				{#each recentVisits as bar}
					<tr>
						<td><a href={toGmapsUrl(bar)}>{bar.name}</a></td>
						<td>{bar.area_name || 'Unknown'}</td>
						<td>{localDate(bar.visited_at ?? '')}</td>
					</tr>
				{/each}
				{#if recentVisitOverflow > 0}
					<tr>
						<td colspan="3">and {recentVisitOverflow} more...</td>
					</tr>
				{/if}
			</tbody>
		</table>
	{:else if userData}
		<p>No recent visits found.</p>
	{/if}

	<h3>Management</h3>
	{#if userData}
		<button on:click={handleLogout} class="outline">Logout</button>
	{/if}
</section>
