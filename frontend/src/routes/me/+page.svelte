<script lang="ts">
	import { user } from '$lib/stores'
	import { onMount } from 'svelte'
	import type { VisitStats, WhoResponse } from '../../models/schemas'
	import { get_bar_visit_stats } from '../../api/session'

	export let userData: WhoResponse | undefined = undefined
	user.subscribe((value) => (userData = value))

	export let visitStats: VisitStats | undefined = undefined

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
	<p>This is you.</p>

	{#if userData}
		<p>Logged in with role '{userData.role}'</p>
	{:else}
		<p>Uhuh not sure who you are, are you even logged in?</p>
	{/if}

	{#if visitStats}
		<p>Visits:</p>
		<ul>
			<li>Total visits: {visitStats.total_bar_visits}</li>
			<li>Distinct bars: {visitStats.distinct_bar_visits}</li>
		</ul>
	{:else}
		<p>Fetching visit stats...</p>
		<progress />
	{/if}
</section>
