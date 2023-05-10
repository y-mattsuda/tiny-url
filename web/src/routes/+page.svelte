<script lang="ts">
	import type { PageData } from './$types';
	import { superForm } from 'sveltekit-superforms/client';
	export let data: PageData;
	const { form, errors, constraints } = superForm(data.form);
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
					type="text"
					name="url"
					data-invalid={$errors.url}
					bind:value={$form.url}
					class="input md:col-span-3"
					placeholder="Shorten your link"
					{...$constraints.url}
				/>
				<button type="submit" class="btn btn-secondary normal-case">Shorten</button>
			</form>
		</div>
	</div>
	<!-- TODO: toastで表示させるように -->
	{#if $errors.url}<span class="alert alert-error shadow-lg">{$errors.url}</span>{/if}
</div>
