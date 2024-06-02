<script lang="ts">
	import { user } from '$lib/stores'
	import type { WhoResponse } from '../../models/schemas'

	export let userData: WhoResponse | undefined = undefined
	user.subscribe((value) => (userData = value))
</script>

<section>
	<h2>Me</h2>
	<p>This is you.</p>

	{#if userData}
		<p>Logged in with role '{userData.role}'</p>
	{:else}
		<p>Uhuh not sure who you are, are you even logged in?</p>
	{/if}
</section>
