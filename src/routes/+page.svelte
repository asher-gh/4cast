<script>
	import Chart from '$lib/Chart.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	async function add() {
		await invoke('add_random');
	}

	async function log() {
		await invoke('log');
	}
</script>

<h1>ICU Bed Forecasts</h1>
<button on:click={add}>add</button>
<button on:click={log}>log</button>
<Chart />

<style>
	:global(body) {
		margin: 0;
		background-color: #100c2a;
	}

	h1 {
		padding: 0 10px;
		color: white;
	}
</style>
