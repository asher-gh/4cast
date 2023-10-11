<script lang="ts">
	import * as echarts from 'echarts';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';

	let chart: HTMLElement;
	let myChart: echarts.ECharts;

	let forecast_algo:
		| undefined
		| {
				name: string;
				mad: number;
				mape: number;
		  };
	export let data:
		| undefined
		| {
				dates: string[];
				beds_actual: number[];
				beds_forecast: number[];
				mad: number;
				mape: number;
		  };

	export function initChart(node: HTMLElement) {
		myChart = myChart || echarts.init(node, 'dark', { height: 500 });

		let option: echarts.EChartsOption = {
			animationDuration: 25000,
			stateAnimation: {
				duration: 300,
				easing: 'cubicOut'
			},
			dataZoom: [
				{
					type: 'slider',
					start: 50,
					end: 100
				},
				{
					type: 'inside',
					start: 0,
					end: 10
				}
			],
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
				bottom: '10%',
				containLabel: true
			},
			toolbox: {
				feature: {
					dataZoom: {
						yAxisIndex: 'none'
					},
					saveAsImage: {}
				}
			},
			xAxis: {
				type: 'category',
				boundaryGap: false,
				data: data ? data.dates : []
			},
			yAxis: {
				type: 'value',
				boundaryGap: [0, '100%']
			},
			series: [
				{
					name: 'Actual',
					showSymbol: false,
					type: 'line',
					data: data ? data.beds_actual : []
				},
				{
					name: 'Forecasted',
					showSymbol: false,
					type: 'line',
					color: '#F2597F',
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
			// TODO: Show progress indicator
			await invoke('read_csv', { csvPath: selected });
			data = await invoke('fetch_data', { shift: 0 });
			initChart(chart);
			forecast_algo = {
				name: 'SMA',
				mad: data!.mad,
				mape: data!.mape
			};
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
<button on:click={() => console.log(data)}>Log fetch response</button>
{#if forecast_algo}
	<pre><code>
   Alogrithm: {forecast_algo.name}
   MAD: {forecast_algo.mad.toFixed(2)}
   MAPE: {forecast_algo.mape.toFixed(2)}%
</code></pre>
{/if}

<style>
	.container {
		width: 100%;
		height: 500px;
	}
	pre {
		color: #ccc;
		font-size: 1.5rem;
	}
</style>
