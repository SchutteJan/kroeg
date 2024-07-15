const dateFormat = new Intl.DateTimeFormat('nl-NL', {})

function localDate(iso8601_utc: string): string {
	const timezoneSuffix = iso8601_utc.includes('Z') ? '' : 'Z'
	const utcDate = new Date(iso8601_utc + timezoneSuffix)
	return dateFormat.format(utcDate)
}

export { localDate }
