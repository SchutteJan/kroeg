export function toGmapsUrl(location: {
	google_place_id?: string | null
	address_line: string
}): string {
	let queryParams: Record<string, string> = {
		api: '1',
		query: location.address_line
	}
	if (location.google_place_id) {
		queryParams = { ...queryParams, query_place_id: location.google_place_id }
	}
	return 'https://www.google.com/maps/search/?' + new URLSearchParams(queryParams).toString()
}
