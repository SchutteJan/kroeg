<script lang="ts">
	import '@picocss/pico';
	import '$lib/pico-settings.css';
	import { user } from '$lib/stores';
	import ThemeSwitcher from '$lib/ThemeSwitcher.svelte';
	import { onMount } from 'svelte';
	import { logout, whoami } from '../api/session';
	import type { WhoResponse } from '../models/schemas';

	export let userData: WhoResponse | undefined = undefined;
	export let darkMode = false;

	user.subscribe((value) => (userData = value));

	async function handleLogout() {
		logout().then(() => {
			user.set(undefined);
		});
	}

	function prefersColorSchemaDark() {
		return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
	}

	function toggleTheme() {
		darkMode = !darkMode;
		document.documentElement.setAttribute('data-theme', darkMode ? 'dark' : 'light');
	}

	onMount(async () => {
		let me = await whoami();
		if (me) {
			user.set(me);
		}

		let dataTheme = document.documentElement.getAttribute('data-theme');
		darkMode = prefersColorSchemaDark() || dataTheme === 'dark';
	});
</script>

<header class="container">
	<div class="container header-container">
		<a href="/" class="contrast header-logo">Kroegen 🍺</a>
		<nav>
			<ul>
				{#if userData}
					<li><a href="/me" class="contrast">Me</a></li>
					<li><a on:click={handleLogout} href="/" class="contrast">Logout</a></li>
				{:else}
					<li><a href="/login" class="contrast">Login</a></li>
					<li><a href="/register" class="contrast">Register</a></li>
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
