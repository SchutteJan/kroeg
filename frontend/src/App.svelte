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
      <li><a href="#" class="contrast">Kroegen</a></li>
      <li><button class="secondary" on:click={() => showLogin = !showLogin}>Login</button></li>
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

