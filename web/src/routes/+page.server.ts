import type { Url, UrlToShorten } from '$lib/types/index.js';
import { error } from '@sveltejs/kit';
import axios from 'axios';

async function postUrlToShort(data: UrlToShorten) {
	return axios
		.post<Url>('http://localhost:8000/shorten', data)
		.then((r) => r.data)
		.catch(() => ({
			error: '😢 Sorry, we failed to shorten your URL.'
		}));
}

export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		const longUrl = data.get('longUrl') as string | null;
		if (longUrl === null) throw error(400, 'form data is empty!');
		const res = await postUrlToShort({
			longUrl
		});
		return res;
	}
};
