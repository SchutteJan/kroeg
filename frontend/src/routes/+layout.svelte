<script>
	import '@picocss/pico';
	import '$lib/pico-settings.css';
</script>

<header class="container">
	<div class="container header-container">
		<a href="/" class="contrast header-logo">Kroegen 🍺</a>
		<nav>
			<ul>
				<li><a href="/login" class="contrast">Login</a></li>
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
