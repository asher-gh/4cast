<script lang="ts">
	import * as echarts from 'echarts';
	import { invoke } from '@tauri-apps/api/tauri';

	type ChartData = {
		x: string[];
		y: number[];
	};

	let data: ChartData = { x: [], y: [] };
	const WINDOW_SIZE = 100;
	const UPDATE_INTERVAL = 500;

	export async function addData(shift = 0) {
		data = await invoke('add_data', { shift });
	}

	// Seed
	for (let i = 1; i <= WINDOW_SIZE; i++) {
		addData();
	}

	let option = {
		// animation: 1000,
		// title: {
		// 	text: 'Forecast time-series',
		// 	x: 'center'
		// },
		tooltip: {
			trigger: 'axis'
		},
		legend: {
			data: ['Actual']
		},
		grid: {
			left: '1%',
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
			data: data.x
		},
		yAxis: {
			type: 'value',
			data: data.y
		},
		series: [
			{
				name: 'Actual',
				showSymbol: false,
				type: 'line',
				data: data.y
			}
		]
	};

	let clear: number;
	let myChart: echarts.ECharts;

	export function charts(node: HTMLElement) {
		myChart = echarts.init(node, 'dark', { height: 500 });
		myChart.setOption(option);
	}

	$: {
		clearInterval(clear);
		clear = setInterval(function () {
			addData(WINDOW_SIZE);
			myChart.setOption({
				xAxis: {
					data: data.x
				},
				series: [
					{
						data: data.y
					}
				]
			});
		}, UPDATE_INTERVAL);
	}
</script>

<svelte:window
	on:resize={() => {
		myChart.resize();
	}}
/>

<div class="container" use:charts />

<style>
	.container {
		width: 100vw;
		height: 500px;
	}
</style>
