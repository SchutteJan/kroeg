import { get_api_base_url } from './base'

export async function get_bars(only_published: boolean = true): Promise<Response> {
	const queryString = new URLSearchParams({ only_published: only_published.toString() }).toString()

	return fetch(get_api_base_url() + '/bars?' + queryString, {
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

export async function deleteVisit(bar_id: number): Promise<Response> {
	return fetch(get_api_base_url() + '/visit/bar/' + bar_id, {
		method: 'DELETE',
		headers: {
			Accept: 'application/json'
		}
	})
}

export async function setPublished(id: number, published: boolean): Promise<Response> {
	return fetch(get_api_base_url() + '/bar/' + id, {
		method: 'PATCH',
		headers: {
			Accept: 'application/json'
		},
		body: JSON.stringify({ published: published })
	})
}
