<script>
	import Chart from '$lib/Chart.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	async function log() {
		await invoke('log');
	}
</script>

<main>
	<h1>ICU Bed Forecasts</h1>
	<Chart />
	<button on:click={log}>Dump data</button>
</main>

<style>
	:global(body) {
		background-color: #100c2a;
		margin: 0;
		padding: 0;
	}

	h1 {
		color: white;
	}

	button {
		margin: 2rem 0 0 0;
		font-weight: 500;
		background: light-grey;
		border: none;
		border-radius: 7px;
		color: black;
		padding: 0.5rem 1rem;
	}
	button:hover {
		cursor: pointer;
	}
	button:active {
		background: white;
	}

	main {
		padding: 10px;
	}
</style>
