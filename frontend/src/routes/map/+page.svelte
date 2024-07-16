<script lang="ts">
	import 'leaflet/dist/leaflet.css'
	import { onMount } from 'svelte'
	import { get_bars } from '../../api/bars'
	import type { LocationResponse } from '../../models/schemas'

	let mapElement: HTMLElement

	function addMapAnnotations(L: any, map: any, bars: Array<LocationResponse>) {
		bars.forEach((bar) => {
			L.circle([bar.coordinates.x, bar.coordinates.y], {
				color: 'red',
				fillColor: '#f03',
				fillOpacity: 0.5,
				radius: 10
			})
				.bindPopup(bar.name)
				.addTo(map)
		})
	}

	onMount(async () => {
		const L = await import('leaflet')

		// Amsterdam center
		const map = L.map(mapElement).setView([52.3694028, 4.9012861], 13)

		// https://www.openstreetmap.fr/fonds-de-carte/
		L.tileLayer('https://b.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
			maxZoom: 19,
			attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
		}).addTo(map)

		get_bars()
			.then((response) => response.json())
			.then((bars) => {
				addMapAnnotations(L, map, bars)
			})
	})
</script>

<section>
	<h2>Map</h2>
	<div bind:this={mapElement} id="map" />
</section>

<style>
	#map {
		height: 500px;
	}
</style>
