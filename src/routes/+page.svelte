<script lang="ts">
	import Chart from '$lib/Chart.svelte';
	// import { invoke } from '@tauri-apps/api/tauri';
	import type { PageData } from './$types';

	export let data: PageData & {
		dates: string[];
		beds_actual: number[];
		beds_forecast: number[];
		mad: number;
		mape: number;
	};

	// async function log() {
	// 	await invoke('log');
	// }
	// function log() {
	// 	console.log(data);
	// }
</script>

<main>
	<h1>ICU Bed Forecasts</h1>
	<Chart {data} />
	<!-- <button on:click={log}>Dump data</button> -->
</main>

<style>
	:global(body) {
		background-color: #100c2a;
		margin: 0;
		padding: 10px;
	}

	:global(button) {
		margin: 2rem 0 0 0;
		font-weight: 500;
		background: light-grey;
		border: none;
		border-radius: 7px;
		color: black;
		padding: 0.5rem 1rem;
	}

	:global(button:hover) {
		cursor: pointer;
	}

	:global(button:active) {
		background: white;
	}

	h1 {
		color: white;
	}
</style>
