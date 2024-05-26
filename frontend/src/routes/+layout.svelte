<script>
	import '@picocss/pico';
	import '$lib/pico-settings.css';
	import { onMount } from 'svelte';
	import { logout, whoami } from '../api/session';

	/**
	 * @type {import("../models/schemas").WhoResponse | undefined}
	 */
	export let user = undefined;

	async function handleLogout() {
		console.log('LOGGING OUT');
		logout().then(() => {
			user = undefined;
		});
	}

	onMount(async () => {
		let me = await whoami();
		if (me) {
			console.log(me);
			user = me;
		}
	});
</script>

<header class="container">
	<div class="container header-container">
		<a href="/" class="contrast header-logo">Kroegen 🍺</a>
		<nav>
			<ul>
				{#if user}
					<li><a href="/me" class="contrast">{user.role}</a></li>
					<li><a on:click={handleLogout} href="/" class="contrast">Logout</a></li>
				{:else}
					<li><a href="/login" class="contrast">Login</a></li>
				{/if}
			</ul>
		</nav>
	</div>
</header>

<main class="container">
	<slot />
</main>

<style>
	.header-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.header-logo {
		text-decoration: none;
	}
</style>
