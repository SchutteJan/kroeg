<script lang="ts">
  import BarItem from "../lib/BarItem.svelte";

  let bars = [];
  
  fetch('http://localhost:8000/bars', {
    method: 'GET',
  }).then(response => response.json())
    .then(json => { bars = json });
  
</script>

<section>
<header class="container">
    <hgroup>
        <h1>All bars</h1>
        <p>
        Below is a list of all bars in Amsterdam
        </p>
    </hgroup>
</header>
<p>
  Here is where that content will live..
</p>

<ul>
  {#each bars as bar, i}
    <BarItem name={bar.name} description={bar.description}/>
  {/each}
</ul>

</section>
