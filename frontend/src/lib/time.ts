function localDate(iso8601_utc: string): string {
	let timezoneSuffix = iso8601_utc.includes('Z') ? '' : 'Z'
	const utcDate = new Date(iso8601_utc + timezoneSuffix)
	return utcDate.toLocaleDateString()
}

export { localDate }
