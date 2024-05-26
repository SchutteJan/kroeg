import { get_api_base_url } from './base';

export async function get_bars(): Promise<Response> {
	return fetch(get_api_base_url() + '/bars', {
		method: 'GET'
	});
}
