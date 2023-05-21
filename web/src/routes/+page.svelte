<script lang="ts">
	export let form;

	let longUrl = '';
	$: shortUrl = form?.shortUrl ? `http://localhost:8000/${form.shortUrl}` : null;

	async function copyToClipboard(value: string) {
		if (navigator.clipboard) {
			await navigator.clipboard.writeText(value);
		} else {
			alert('このブラウザでは`navigator.clipboard`が存在しないようです...');
		}
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div>
	<div class="p-20 flex justify-center">
		<h1 class="text-5xl font-extrabold">
			Welcome to Tiny URL!<br />
			Shorten URLs. Generate QR Codes.
		</h1>
	</div>
	<div class="bg-secondary-content">
		<div class="p-4">
			{#if form === null || form.error !== undefined}
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
					<button type="submit" class="btn btn-secondary normal-case text-lg">Shorten</button>
				</form>
			{:else}
				<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
					<input value={shortUrl} class="input md:col-span-3 text-secondary-content px-8 text-lg" />
					<button
						on:click={() => {
							if (typeof shortUrl === 'string') {
								copyToClipboard(shortUrl);
							}
						}}
						class="btn btn-secondary normal-case text-lg">Copy</button
					>
					<div class="card bg-white rounded-md md:col-span-3">
						<div class="card-body">
							<p class="text-lg">{form.longUrl}</p>
							<a href={shortUrl} target="_blank" class="link link-primary text-lg">{shortUrl}</a>
						</div>
					</div>
					<button on:click={() => location.reload()} class="btn btn-secondary normal-case text-lg"
						>Again?</button
					>
				</div>
			{/if}
		</div>
	</div>
	{#if form !== null && form.error !== undefined}
		<div class="toast">
			<div class="alert alert-error">
				<div>
					<span>{form.error}</span>
				</div>
			</div>
		</div>
	{/if}
</div>
