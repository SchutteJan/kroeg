<script lang="ts">
	import type { LocationResponse } from '../models/schemas';

	export let bar: LocationResponse;
</script>

<div>
	<b>Name:</b>
	{bar.name} <br />
	<b>Description:</b>
	{bar.description ?? 'No Description'} <br />
	<b>Coords:</b>
	x: {bar.coordinates.x}, y: {bar.coordinates.y}
</div>
