<script lang="ts">
  import BarItem from "../lib/BarItem.svelte"

  let bars = []
  let loading = true
  
  fetch('http://localhost:8000/bars', {
    method: 'GET',
  }).then(response => {
    loading = false
    return response.json()
  }).then(json => { bars = json });
  
</script>

<section>
<header class="container">
    <hgroup>
        <h1>Kroegen</h1>
        <p>
        Zie hier een lijst van alle kroegen in Amsterdam.
        </p>
    </hgroup>
</header>


<div class="bar-list-container">
  {#if loading}
    <progress/>
  {:else}
    {#each bars as bar, i}
      <BarItem name={bar.name} description={bar.description}/>
    {/each}
  {/if}
</div>

</section>
