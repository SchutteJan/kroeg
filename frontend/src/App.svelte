<script lang="ts">
import Login from './routes/Login.svelte'
import Home from './routes/Home.svelte'
import { Router, Link, Route } from 'svelte-routing'

const url = "/"
</script>

<Router {url}>
  <nav class="container-fluid">
    <ul>
      <li><Link to="/">🍺</Link></li>
    </ul>
    <ul>
      <li><Link class="contrast" to="/login">Login</Link></li>
    </ul>
</nav>
<main class="container">
    <Route path="/" component={Home} />
    <Route path="/login" component={Login} />
</main>
</Router>