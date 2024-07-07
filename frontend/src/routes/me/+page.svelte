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

	{#if visitStats && userData}
		<p>Visits:</p>
		<ul>
			<li>Total bars visited: {visitStats.distinct_bar_visits}</li>
		</ul>
	{:else if userData}
		<p>Fetching visit stats...</p>
		<progress />
	{/if}

	{#if userData}
		<button on:click={handleLogout} class="outline">Logout</button>
	{/if}
</section>
