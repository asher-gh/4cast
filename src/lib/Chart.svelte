<script lang="ts">
	import * as echarts from 'echarts';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';

	let chart: HTMLElement;
	let myChart: echarts.ECharts;
	export let data:
		| undefined
		| {
				dates: string[];
				beds_actual: number[];
				beds_forecast: number[];
		  };

	export function initChart(node: HTMLElement) {
		myChart = myChart || echarts.init(node, 'dark', { height: 500 });

		let option: echarts.EChartsOption = {
			animationDuration: 10000,
			tooltip: {
				trigger: 'axis',
				order: 'valueDesc'
			},
			legend: {
				data: ['Actual', 'Forecasted']
			},
			grid: {
				left: '0',
				right: '1%',
				bottom: '3%',
				containLabel: true
			},
			toolbox: {
				feature: {
					saveAsImage: {}
				}
			},
			xAxis: {
				type: 'category',
				boundaryGap: false,
				data: data ? data.dates : []
			},
			yAxis: {
				type: 'value'
			},
			series: [
				{
					name: 'Actual',
					showSymbol: false,
					type: 'line',
					// stack: 'a',
					data: data ? data.beds_actual : []
				},
				{
					name: 'Forecasted',
					showSymbol: false,
					type: 'line',
					// stack: 'a',
					color: 'red',
					data: data ? data.beds_forecast : []
				}
			]
		};
		myChart.setOption(option);
	}

	async function openCSV() {
		const selected = await open({
			multiple: false,
			filters: [
				{
					name: 'Data',
					extensions: ['csv']
				}
			]
		});
		console.log(selected);

		if (!Array.isArray(selected) && selected != null) {
			await invoke('read_csv', { csvPath: selected });
			data = await invoke('fetch_data', { shift: 0 });
			initChart(chart);
		}
	}
</script>

<svelte:window
	on:resize={() => {
		myChart.resize();
	}}
/>

<div class="container" bind:this={chart} use:initChart />
<button on:click={openCSV}>Load</button>

<style>
	.container {
		width: 100%;
		height: 500px;
	}
</style>
