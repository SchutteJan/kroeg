<script lang="ts">
	import type { Coordinate } from '../models/schemas';

	export let name: string;
	export let description: string;
	export let coordinates: Coordinate;
</script>

<div>
	<b>Name:</b>
	{name} <br />
	<b>Description:</b>
	{description} <br />
	<b>Coords:</b>
	x: {coordinates.x}, y: {coordinates.y}
</div>
