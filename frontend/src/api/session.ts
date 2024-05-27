import type { WhoResponse } from '../models/schemas';
import { get_api_base_url } from './base';

export async function whoami(): Promise<WhoResponse | undefined> {
	return fetch(get_api_base_url() + '/session/who', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then(async (response) => {
		switch (response.status) {
			case 200: {
				return await await response.json();
			}
			case 401:
				return undefined;
		}
	});
}

export async function logout(): Promise<void> {
	return fetch(get_api_base_url() + '/session/logout', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then(() => {});
}

export async function login(form: FormData): Promise<Response> {
	return fetch(get_api_base_url() + '/session/login', {
		method: 'POST',
		headers: {
			Accept: 'application/json'
		},
		body: form
	});
}

export async function register(form: FormData): Promise<Response> {
	return fetch(get_api_base_url() + '/session/create', {
		method: 'POST',
		headers: {
			Accept: 'application/json'
		},
		body: form
	});
}
