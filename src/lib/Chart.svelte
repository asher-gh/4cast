<script lang="ts">
	import * as echarts from 'echarts';
	let base = +new Date(2014, 9, 3);
	let oneDay = 24 * 3600 * 1000;
	let date: string[] = [];
	var data = [Math.random() * 150];
	let now = new Date(base);
	function addData(shift: boolean = false) {
		// now = new Date([now.getFullYear(), now.getMonth() + 1, now.getDate()].join('/'));
		let nowf = [now.getFullYear(), now.getMonth() + 1, now.getDate()].join('/');
		date.push(nowf);
		data.push((Math.random() - 0.4) * 10 + data[data.length - 1]);
		if (shift) {
			date.shift();
			data.shift();
		}
		now = new Date(+new Date(now) + oneDay);
	}

	// seed data
	for (let i = 1; i < 100; i++) {
		addData();
	}

	const option = {
		title: {
			text: 'Time series forecast using Simple Moving Average',
			padding: 15,
			x: 'center'
		},
		xAxis: {
			type: 'category',
			boundaryGap: false,
			data: date
		},
		yAxis: {
			boundaryGap: [0, '50%'],
			type: 'value'
		},
		series: [
			{
				name: '成交',
				type: 'line',
				// smooth: true,
				symbol: 'none',
				stack: 'a',
				data: data
			}
		]
	};

	let clear: number;
	let myChart: echarts.ECharts;

	$: {
		clearInterval(clear);
		clear = setInterval(function () {
			addData(true);
			myChart.setOption({
				xAxis: {
					data: date
				},
				series: [
					{
						name: '成交',
						data: data
					}
				]
			});
		}, 500);
	}

	export function charts(node: HTMLElement) {
		myChart = echarts.init(node, 'dark', { height: 500 });
		myChart.setOption(option);
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
		margin: 0;
		padding: 0;
	}
</style>
