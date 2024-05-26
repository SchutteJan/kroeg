<!-- TODO: this script is not typescript? -->
<script>
	import '@picocss/pico';
	import '$lib/pico-settings.css';
	import ThemeSwitcher from '$lib/ThemeSwitcher.svelte';
	import { onMount } from 'svelte';
	import { logout, whoami } from '../api/session';

	/**
	 * @type {import("../models/schemas").WhoResponse | undefined}
	 */
	export let user = undefined;
	export let darkMode = false;

	async function handleLogout() {
		logout().then(() => {
			user = undefined;
		});
	}

	function prefersColorSchemaDark() {
		return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
	}

	function toggleTheme() {
		darkMode = !darkMode;
		// <html data-theme="light">
		document.documentElement.setAttribute('data-theme', darkMode ? 'dark' : 'light');
	}

	onMount(async () => {
		let me = await whoami();
		if (me) {
			console.log(me);
			user = me;
		}

		if (prefersColorSchemaDark()) {
			darkMode = true;
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
				<li><ThemeSwitcher onClick={toggleTheme} {darkMode} /></li>
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
