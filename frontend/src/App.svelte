<script lang="ts">
import LoginForm from './lib/LoginForm.svelte'

let showLogin: boolean = false
</script>

{#if showLogin}
<dialog open>
    <LoginForm />
</dialog>
{/if}

<nav class="container-fluid">
    <ul>
      <li>🍺</li>
    </ul>
    <ul>
      <li><a href="#">Kroegen</a></li>
      <li><a href="#" role="button" on:click={() => showLogin = !showLogin}>Login</a></li>
    </ul>
  </nav>

<header class="container">
    <hgroup>
        <h1>Alle kroegen</h1>
        <p>
        Zie hier beneden een lijst van alle kroegen in Amsterdam
        </p>
    </hgroup>
</header>

