<script>
	import '@picocss/pico';
	import '$lib/pico-settings.css';
</script>

<nav class="container-fluid">
	<ul>
		<li><a href="/">🍺</a></li>
		<li><a href="/login">Login</a></li>
	</ul>
</nav>

<main class="container">
	<slot />
</main>
