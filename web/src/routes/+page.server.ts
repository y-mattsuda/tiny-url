import type { Url, UrlToShorten } from '$lib/types/index.js';
import { fail } from '@sveltejs/kit';
import axios from 'axios';

async function postUrlToShort(data: UrlToShorten) {
	return axios
		.post<Url>('http://localhost:8000/shorten', data)
		.then((r) => r.data)
		.catch(() => ({
			error: 'Sorry, we failed to shorten your Long URL'
		}));
}

export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		console.log('POST', data);
		const longUrl = data.get('longUrl') as string | null;
		if (longUrl === null) return fail(400);
		const res = await postUrlToShort({
			longUrl
		});
		console.log('res', res);
		return res;
	}
};
