<script lang="ts">
	export let form;

	let longUrl = '';

	const host = 'http://localhost:8000/';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div>
	<div class="p-20 flex justify-center">
		<h1 class="text-4xl font-extrabold">
			Welcome to Tiny URL!<br />
			Shorten URLs. Generate QR Codes.
		</h1>
	</div>
	<div class="bg-secondary-content">
		<div class="p-4">
			<form method="POST" class="grid grid-cols-1 md:grid-cols-4 gap-4">
				<input
					type="url"
					name="longUrl"
					bind:value={longUrl}
					class="input md:col-span-3"
					placeholder="Shorten your link"
					required
					autocomplete="off"
				/>
				<button type="submit" class="btn btn-secondary normal-case">Shorten</button>
			</form>
		</div>
	</div>
	<!-- TODO: toastで表示させるように -->
	{#if form !== null}
		{#if form?.error}<span class="alert alert-error shadow-lg">{form.error}</span>
		{:else}
			<p>
				短縮後のURLは<a href="{host}{form.shortUrl}" target="_blank">{host}{form.shortUrl}</a>だよ!
			</p>
		{/if}
	{/if}
</div>
