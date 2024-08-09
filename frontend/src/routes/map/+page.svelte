<script lang="ts">
	import 'leaflet/dist/leaflet.css'
	import { onMount } from 'svelte'
	import { get_bars } from '../../api/bars'
	import { get_areas } from '../../api/areas'
	import type { Area, LocationResponse } from '../../models/schemas'
	import MapBarItem from '$lib/MapBarItem.svelte'

	let mapElement: HTMLElement
	const GREEN = '#00895A'
	const RED = '#D93526'

	function renderPopup(layer: any, bar: LocationResponse) {
		let container = document.createElement('div')
		new MapBarItem({
			target: container,
			props: {
				bar: bar,
				onVisitCallback: () => {
					layer._path.style.stroke = GREEN
					layer._path.style.fill = GREEN
				},
				onDeleteVisitCallback: () => {
					layer._path.style.stroke = RED
					layer._path.style.fill = RED
				}
			}
		})
		return container
	}

	function addBarMapAnnotations(L: any, map: any, bars: Array<LocationResponse>) {
		bars.forEach((bar) => {
			L.circle([bar.coordinates.x, bar.coordinates.y], {
				color: bar.visited_at ? GREEN : RED,
				fillOpacity: 0.2,
				radius: 7
			})
				.bindPopup((layer: any) => {
					return renderPopup(layer, bar)
				})
				.bringToFront()
				.addTo(map)
		})
	}

	function addAreaAnnotations(L: any, map: any, areas: Array<Area>) {
		areas.forEach((area) => {
			L.polygon(
				area.area.rings.map((ring) => ring.map((point) => [point.x, point.y])),
				// #0172ad is equivalent to --pico-primary
				{ color: '#0172ad', fill: false }
			)
				.bindPopup(area.name)
				.addTo(map)
		})
	}

	onMount(async () => {
		const L = await import('leaflet')

		// Amsterdam center
		const map = L.map(mapElement).setView([52.3694028, 4.9012861], 13)

		// https://api.data.amsterdam.nl/api/
		L.tileLayer('https://t2.data.amsterdam.nl/topo_wm_light/{z}/{x}/{y}.png', {
			maxZoom: 21,
			attribution: '&copy; Gemeente Amsterdam'
		}).addTo(map)

		// Load area polygons
		get_areas()
			.then((response) => response.json())
			.then((areas) => {
				addAreaAnnotations(L, map, areas)
			})

		// Load bar annotations
		get_bars()
			.then((response) => response.json())
			.then((bars) => {
				addBarMapAnnotations(L, map, bars)
			})
	})
</script>

<section>
	<h2>Map</h2>
	<div bind:this={mapElement} id="map" />
</section>

<style>
	#map {
		height: 75vh;
	}
</style>
