<template>
	<!-- <div class="echarts"> -->
	<div ref="chartRef" style="width: 100%; height: 100%"></div>
	<!-- </div> -->
</template>

<script setup>
import * as echarts from 'echarts';

import elementResizeDetectorMaker from 'element-resize-detector';

var erd = elementResizeDetectorMaker();

const props = defineProps({
	option: {
		type: Object,
		default: () => {},
	},
});

const chartRef = ref(null);
let myChart;

onMounted(() => {
	renderChart();
});

onBeforeUnmount(() => {
	erd.removeListener(chartRef.value, resize);
	window.removeEventListener('resize', resize);
	myChart.dispose();
});

const renderChart = () => {
	nextTick(() => {
		myChart = echarts.init(chartRef.value);
		myChart.setOption(props.option);
		window.addEventListener('resize', resize);
		erd.listenTo(chartRef.value, resize);
	});
};

const resize = () => {
	myChart.resize();
};

defineExpose({ echarts });

watch(
	() => props.option,
	(val) => {
		myChart.setOption(val, true);
	},
	{ deep: true },
);
</script>

<style lang="scss" scoped>
//.echarts {
//	width: 100%;
//	height: 100%;
//	display: flex;
//	align-items: center;
//	justify-content: center;
//}
</style>
