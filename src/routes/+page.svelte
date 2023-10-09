<script lang="ts">
	import Chart from '$lib/Chart.svelte';
	import type { PageData } from './$types';

	export let data: PageData & {
		dates: string[];
		beds_actual: number[];
		beds_forecast: number[];
		mad: number;
		mape: number;
	};
</script>

<main>
	<h1>ICU Bed Demand Forecast</h1>
	<Chart {data} />
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
