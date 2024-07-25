import { get_api_base_url } from './base'

export async function get_bars(): Promise<Response> {
	return fetch(get_api_base_url() + '/bars', {
		method: 'GET'
	})
}

export async function visitBar(id: number): Promise<Response> {
	return fetch(get_api_base_url() + '/visit/bar/' + id, {
		method: 'POST',
		headers: {
			Accept: 'application/json'
		}
	})
}

export async function hideBar(id: number): Promise<Response> {
	return fetch(get_api_base_url() + '/bar/' + id, {
		method: 'PATCH',
		headers: {
			Accept: 'application/json'
		},
		body: JSON.stringify({ published: false })
	})
}
