import { get_api_base_url } from './base'

export async function get_areas(): Promise<Response> {
	return fetch(get_api_base_url() + '/areas', {
		method: 'GET',
		headers: {
			Accept: 'application/json'
		}
	})
}
