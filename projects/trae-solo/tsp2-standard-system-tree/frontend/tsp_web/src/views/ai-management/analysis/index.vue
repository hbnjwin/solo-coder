<template>
	<div class="analysis-layout">
		<div class="top"></div>
		<div style="padding: 0 20px">
			<el-row :gutter="20" class="middle">
				<el-col :span="6" v-for="(item, index) in list" :key="index">
					<div class="box flex-box">
						<img :src="item.img" alt="" />
						<div class="text flex-col">
							<p class="title">{{ item.title }}</p>
							<p class="desc">{{ item.desc }}</p>
						</div>
					</div>
				</el-col>
			</el-row>
			<el-row :gutter="20" class="bottom">
				<el-col :span="7" class="flex-col">
					<div class="circle">
						<div class="ctop">
							<img src="@/assets/image/ai/anlysis/箭头@2x.png" alt="" />
							训练文档变化趋势
						</div>
						<div class="cbottom">
							<Echarts style="height: 100%; width: 100%"  :option="form.lineOptions"></Echarts>
						</div>
					</div>
					<div class="circle">
						<div class="ctop">
							<img src="@/assets/image/ai/anlysis/箭头@2x.png" alt="" />
							智能检索请求调用统计、使用时间段
						</div>
						<div class="cbottom">
							<Echarts style="height: 100%; width: 100%" :option="form.lineOptions2"></Echarts>
						</div>
					</div>
				</el-col>
				<el-col :span="10">
					<div class="circle" style="height: 99%">
						<div class="ctop">
							<img src="@/assets/image/ai/anlysis/箭头@2x.png" alt="" />
							河南省地市标准数量统计
						</div>
						<div class="cbottom">
							<div class="graph-box">
								<div class="box flex-box">
									<img src="@/assets/image/ai/anlysis/标准文件数量@2x.png" alt="" />
									<div class="text flex-col">
										<p class="title" style="color: #009fe9">{{ graphForm.standardDocument }}</p>
										<p class="desc">标准文件数量</p>
									</div>
								</div>
								<div class="box flex-box">
									<img src="@/assets/image/ai/anlysis/AI智能体使用次数@2x.png" alt="" />
									<div class="text flex-col">
										<p class="title" style="color: #00ab84">{{ graphForm.aiAgentCount }}</p>
										<p class="desc">AI智能体使用次数</p>
									</div>
								</div>
							</div>
							<div style="height: 80%; width: 100%">
								<Echarts style="height: 100%; width: 100%" :option="form.mapOptions"></Echarts>
							</div>
						</div>
					</div>
				</el-col>
				<el-col :span="7" class="flex-col">
					<div class="circle">
						<div class="ctop">
							<img src="@/assets/image/ai/anlysis/箭头@2x.png" alt="" />
							AI知识库问答使用统计
						</div>
						<div class="select">
							<el-radio-group v-model="radio" size="small" fill="#6cf">
								<el-radio-button label="按月" value="month" />
								<el-radio-button label="按日" value="day" />
							</el-radio-group>
						</div>
						<div class="cbottom">
							<Echarts style="height: 100%; width: 100%" :option="form.lineOptions3"></Echarts>
						</div>
					</div>
					<div class="circle">
						<div class="ctop">
							<img src="@/assets/image/ai/anlysis/箭头@2x.png" alt="" />
							用户问答反馈统计
						</div>
						<div class="cbottom pie">
							<div class="total">
								<Echarts style="height: 100%; width: 100%"  :option="form.lineOptions4"></Echarts>
							</div>
							<div class="first">
								<div class="seacon flex-box">
									<img src="@/assets/image/ai/anlysis/green.png" />
									<span style="min-width: 36px">{{ GraFormName[0] }}</span>
									<span>{{ GraFormNum[0] }}</span>
									<span>{{ GraFormPre[0] }}</span>
								</div>
								<div class="seacon flex-box">
									<img src="@/assets/image/ai/anlysis/orange.png" />
									<span style="min-width: 36px">{{ GraFormName[1] }}</span>
									<span>{{ GraFormNum[1] }}</span>
									<span>{{ GraFormPre[1] }}</span>
								</div>
								<div class="seacon flex-box">
									<img src="@/assets/image/ai/anlysis/blue.png" />
									<span style="min-width: 36px">{{ GraFormName[2] }}</span>
									<span>{{ GraFormNum[2] }}</span>
									<span>{{ GraFormPre[2] }}</span>
								</div>
							</div>
						</div>
					</div>
				</el-col>
			</el-row>
		</div>
	</div>
</template>

<script setup>
import { require } from '@/utils/require.js';
import Echarts from '@/components/echarts/index.vue';
import * as echarts from 'echarts';
import { ref, onMounted } from 'vue';
import { ANALYSIS } from '@/api';
import { tool } from '@/utils';
const radio = ref('month');
import henanJson from './henan.json';
const list = ref([
	{
		img: require('ai/anlysis/累计标准文件总数@2x.png'),
		title: '48,842',
		desc: '累计标准文件总数',
	},
	{
		img: require('ai/anlysis/标准检索次数@2x.png'),
		title: '85,541',
		desc: '标准检索次数',
	},
	{
		img: require('ai/anlysis/模型训练文件数量@2x.png'),
		title: '113,683',
		desc: '模型训练文件数量',
	},
	{
		img: require('ai/anlysis/AI知识库问答总次数@2x.png'),
		title: '101,540',
		desc: 'AI知识库问答总次数',
	},
]);
const graphForm = ref({
	standardDocument: '',
	aiAgentCount: '',
});
const GraFormName = ref([]);
const GraFormNum = ref([]);
const GraFormPre = ref([]);
const form = reactive({
	lineOptions: {},
	lineOptions2: {},
	lineOptions3: {},
	lineOptions4: {},
	mapOptions: {},
});
const monthStatic = ref('');
const dayStatic = ref('');
watch(radio, (value) => {
	value === 'month' ? dismant(form.lineOptions3, monthStatic.value) : dismant(form.lineOptions3, dayStatic.value, 1);
});
const dismant = (weather, jsonString, type) => {
	const arrayList = !type ? tool.arrayDismant(jsonString) : tool.stringDismant(jsonString);
	if (weather === 'lineOptions') {
		form.lineOptions = renderLineChart(arrayList.key, arrayList.value);
	} else if (weather === 'lineOptions2') {
		form.lineOptions2 = renderLineChart2(arrayList.key, arrayList.value);
	} else if (weather === 'lineOptions3') {
		form.lineOptions3 = renderLineChart3(arrayList.key, arrayList.value);
	}
};
const changeGra = (jsonString) => {
	const arrayList = tool.arrayDismant(jsonString);
	GraFormName.value = arrayList.key;
	GraFormNum.value = arrayList.value;
	let totel = 0;
	arrayList.value.forEach((item) => {
		totel += item;
	});
	GraFormPre.value = arrayList.value.map((item) => {
		return ((item / totel) * 100).toFixed(2) + '%';
	});
	let seriesData = [];
	GraFormNum.value.forEach((item, index) => {
		seriesData.push({
			value: GraFormName.value[index],
			name: GraFormName.value[index],
			itemStyle: { color: index === 0 ? '#5cb85c' : index === 1 ? '#f0ad4e' : '#5bc0de' },
		});
	});
	form.lineOptions4 = renderLineChart4(seriesData, totel);
	form.lineOptions4.series[0].data.forEach((item, index) => {
		item.value = GraFormNum.value[index];
		item.name = GraFormName.value[index];
	});
	form.lineOptions4.graphic[0].style.text = totel;
};
const getStaticData = () => {
	ANALYSIS.getStatisticsData()
		.then(({ data }) => {
			const { deptStandardDocData, deptAiAgentData, trainingDocMonthlyData, smartSearchTimeData, aiQueryMonthlyData } = data.data;
			list.value[0].title = data.data.standardDocCount;
			list.value[1].title = data.data.standardCheckCount;
			list.value[2].title = data.data.trainingDocCount;
			list.value[3].title = data.data.aiQueryTotalCount;
			graphForm.value.standardDocument = data.data.standardDocCount;
			graphForm.value.aiAgentCount = data.data.aiAgentCount;
			renderMapData(deptStandardDocData, deptAiAgentData); //数量统计
			dismant('lineOptions', data.data.trainingDocMonthlyData); //训练文档
			dismant('lineOptions2', data.data.smartSearchTimeData); //时间段
			dismant('lineOptions3', data.data.aiQueryMonthlyData); //使用统计
			monthStatic.value = data.data.aiQueryMonthlyData;
			dayStatic.value = data.data.aiQueryDayData;
			changeGra(data.data.userQueryTypeData);
		})
		.catch((err) => {
			console.log(err);
		});
};
const renderMapData = (deptStandardDocData, deptAiAgentData) => {
	let deptAiAgentDataNew = [];
	Object.keys(JSON.parse(deptAiAgentData)).forEach((item) => {
		const cityIndex = item.indexOf('市');
		if (cityIndex !== -1) {
			deptAiAgentDataNew.push({ name: item.substring(0, cityIndex), value: JSON.parse(deptAiAgentData)[item] });
		}
	});
	let deptStandardDocDataNew = [];
	Object.keys(JSON.parse(deptStandardDocData)).forEach((item) => {
		const cityIndex = item.indexOf('市');
		if (cityIndex !== -1) {
			deptStandardDocDataNew.push({ name: item.substring(0, cityIndex), value: JSON.parse(deptStandardDocData)[item] });
		}
	});
	// 数据关联
	const dataMap = {};
	deptStandardDocDataNew.forEach((item) => (dataMap[item.name] = { ...item, aiValue: null }));
	deptAiAgentDataNew.forEach((item) => {
		if (dataMap[item.name]) {
			dataMap[item.name].aiValue = item.value;
		}
	});

	const series1Data = deptStandardDocDataNew.map((item) => ({
		...item,
		aiValue: dataMap[item.name]?.aiValue,
	}));

	const series2Data = deptAiAgentDataNew.map((item) => ({
		...item,
		standerValue: dataMap[item.name]?.value,
	}));
	form.mapOptions = renderMapChart(series1Data, series2Data);
};
onMounted(() => {
	// form.lineOptions4 = renderLineChart4();
	getStaticData();
});

const renderMapChart = (series1Data, series2Data) => {
	echarts.registerMap('henan', henanJson);
	return {
		title: {
			text: '',
			left: 'right',
		},
		tooltip: {
			trigger: 'item',
			showDelay: 0,
			transitionDuration: 0.2,
			formatter: function (params) {
				const stander = params.data ? params.data.value : 0;
				const ai = params.data ? params.data.aiValue : 0;
				return `${params.name}<br>标准文件: ${stander}次<br>AI使用: ${ai}次`;
			},
		},
		visualMap: [
			{
				id: 'stander',
				left: 'right',
				min: 0,
				max: 30, // 标准数量
				inRange: { color: ['#313695', '#4575b4', '#74add1', '#abd9e9', '#e0f3f8', '#ffffbf', '#fee090', '#fdae61', '#f46d43', '#d73027', '#a50026'] },
				text: ['High', 'Low'],
			},
			{
				id: 'ai',
				right: 'right',
				min: 0,
				max: 30, // ai数量
				inRange: { color: ['#313695', '#4575b4', '#74add1', '#abd9e9', '#e0f3f8', '#ffffbf', '#fee090', '#fdae61', '#f46d43', '#d73027', '#a50026'] },
				text: ['High', 'Low'],
			},
		],
		toolbox: {
			show: false,
		},
		series: [
			{
				name: '标准文件数量',
				type: 'map',
				map: 'henan',
				roam: true,
				label: { show: true },
				data: series1Data,
				visualMap: 'stander',
			},
			{
				name: 'AI智能体使用次数',
				type: 'map',
				map: 'henan',
				roam: false,
				label: { show: false },
				itemStyle: { opacity: 0.7 },
				data: series2Data,
				visualMap: 'ai',
				zIndex: 10,
			},
		],
	};
};

const renderLineChart3 = (xAxisData, seriesData) => {
	return {
		title: {
			subtext: '数量',
		},
		tooltip: {
			trigger: 'axis',
			formatter: '{b}: {c}',
		},
		grid: {
			left: '3%',
			right: '4%',
			bottom: '3%',
			top: '15%',
			containLabel: true,
		},
		xAxis: {
			type: 'category',
			data: xAxisData,
			axisLine: {
				lineStyle: {
					color: '#333',
				},
			},
		},
		yAxis: {
			type: 'value',
			min: 0,
			interval: 50,
			axisLabel: {
				formatter: '{value}',
			},
			splitLine: {
				lineStyle: {
					type: 'dashed',
				},
			},
		},
		series: [
			{
				name: '数量',
				type: 'line',
				smooth: true, // 平滑曲线
				symbol: 'circle', // 数据点圆形
				symbolSize: 8, // 数据点大小
				lineStyle: {
					width: 4, // 线条加粗
					color: '#EEA547', // 蓝色线条
				},
				itemStyle: {
					color: '#fff', // 数据点颜色
					borderWidth: 2,
					borderColor: '#EEA547', // 白色边框使数据点更明显
				},
				data: seriesData, // 示例数据
				areaStyle: {
					color: {
						type: 'linear',
						x: 0,
						y: 0,
						x2: 0,
						y2: 1,
						colorStops: [
							{
								offset: 0,
								color: 'rgba(238, 165, 71, 0.5)', // 顶部透明度
							},
							{
								offset: 1,
								color: 'rgba(84, 112, 198, 0.01)', // 底部透明度
							},
						],
					},
				},
			},
		],
	};
};
const renderLineChart2 = (xAxisData, seriesData) => {
	return {
		title: {
			subtext: '次数',
		},
		tooltip: {
			trigger: 'axis',
			formatter: '{b}: {c}',
		},
		grid: {
			left: '3%',
			right: '4%',
			bottom: '3%',
			top: '15%',
			containLabel: true,
		},
		xAxis: {
			type: 'category',
			data: xAxisData,
			axisLine: {
				lineStyle: {
					color: '#333',
				},
			},
		},
		yAxis: {
			type: 'value',
			min: 0,
			interval: 30,
			axisLabel: {
				formatter: '{value}',
			},
			splitLine: {
				lineStyle: {
					type: 'dashed',
				},
			},
		},
		series: [
			{
				name: '数量',
				type: 'line',
				smooth: true, // 平滑曲线
				symbol: 'circle', // 数据点圆形
				symbolSize: 8, // 数据点大小
				lineStyle: {
					width: 4, // 线条加粗
					color: '#00AB84', // 蓝色线条
				},
				itemStyle: {
					color: '#fff', // 数据点颜色
					borderWidth: 2,
					borderColor: '#00AB84', // 白色边框使数据点更明显
				},
				data: seriesData, // 示例数据
				areaStyle: {
					color: {
						type: 'linear',
						x: 0,
						y: 0,
						x2: 0,
						y2: 1,
						colorStops: [
							{
								offset: 0,
								color: 'rgba(0, 171, 132, 0.3)', // 顶部透明度
							},
							{
								offset: 1,
								color: 'rgba(84, 112, 198, 0.01)', // 底部透明度
							},
						],
					},
				},
			},
		],
	};
};
const renderLineChart4 = (seriesData, totel) => {
	return {
		tooltip: {
			trigger: 'item',
			formatter: '{b}: {c} ({d}%)',
		},
		series: [
			{
				name: '数据统计',
				type: 'pie',
				radius: ['50%', '80%'],
				center: ['50%', '50%'],
				avoidLabelOverlap: false,
				itemStyle: {
					borderRadius: 8,
					borderColor: '#fff',
					borderWidth: 3,
				},
				label: {
					show: false,
				},
				emphasis: {
					label: {
						show: false,
					},
					itemStyle: {
						shadowBlur: 10,
						shadowOffsetX: 0,
						shadowColor: 'rgba(0, 0, 0, 0.5)',
					},
				},
				labelLine: {
					show: false,
				},
				data: seriesData,
			},
		],
		graphic: [
			{
				type: 'text',
				left: 'center',
				top: '40%',
				style: {
					text: totel,
					fontSize: 26,
					fontWeight: 'bold',
					fill: '#2c3e50',
				},
			},
			{
				type: 'text',
				left: 'center',
				top: '55%',
				style: {
					text: '总计（条）',
					fontSize: 15,
					fill: '#7f8c8d',
				},
			},
		],
	};
};
const renderLineChart = (xAxisData, seriesData) => {
	return {
		title: {
			subtext: '数量',
		},
		tooltip: {
			trigger: 'axis',
			formatter: '{b}: {c}',
		},
		grid: {
			left: '3%',
			right: '4%',
			bottom: '3%',
			top: '15%',
			containLabel: true,
		},
		xAxis: {
			type: 'category',
			data: xAxisData,
			axisLine: {
				lineStyle: {
					color: '#333',
				},
			},
		},
		yAxis: {
			type: 'value',
			min: 0,
			interval: 20,
		},
		series: [
			{
				name: '数量',
				type: 'line',
				smooth: true, // 平滑曲线
				symbol: 'circle', // 数据点圆形
				symbolSize: 8, // 数据点大小
				lineStyle: {
					width: 4, // 线条加粗
					color: '#0078E9', // 蓝色线条
				},
				itemStyle: {
					color: '#fff', // 数据点颜色
					borderWidth: 2,
					borderColor: '#0078E9', // 白色边框使数据点更明显
				},
				data: seriesData, // 示例数据
				areaStyle: {
					color: {
						type: 'linear',
						x: 0,
						y: 0,
						x2: 0,
						y2: 1,
						colorStops: [
							{
								offset: 0,
								color: 'rgba(84, 112, 198, 0.3)', // 顶部透明度
							},
							{
								offset: 1,
								color: 'rgba(84, 112, 198, 0.01)', // 底部透明度
							},
						],
					},
				},
			},
		],
	};
};
</script>

<style lang="less" scoped>
.analysis-layout {
	// min-width: 1697px;
	width: 100%;
	height: calc(100vh - 150px);
	overflow-y: auto;
	.flex-box {
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.flex-col {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
	}
	.top {
		background-image: url('@/assets/image/ai/anlysis/title@2x.png');
		background-size: 100% 100%;
		width: 100%;
		height: 8vh;
		margin-bottom: 20px;
	}
	.middle {
		.box {
			padding: 10px 0;
			border-radius: 8px;
			background: linear-gradient(180deg, rgba(255, 255, 255, 0.76) 0%, rgba(222, 239, 255, 0.76) 100%);
			border: 2px solid #ffffff;
			height: 10vh;
			img {
				height: 100%;
				width: auto;
				margin-right: 20px;
			}
			.text {
				height: 60%;
				.title {
					font-size: 22px;
					font-weight: 900;
					color: #009fe9;
				}
				.desc {
					font-size: 16px;
					font-weight: bold;
					color: #3d3d3d;
				}
			}
		}
	}
	.bottom {
		min-height: 720px;
		padding: 20px 0;

		.circle {
			width: 100%;
			height: 47%;
			border-radius: 8px;
			background: linear-gradient(180deg, rgba(255, 255, 255, 0.76) 0%, rgba(222, 239, 255, 0.76) 100%);
			border: 2px solid #ffffff;
			overflow: auto;
			position: relative;
			.ctop {
				border-radius: 8px 8px 0px 0px;
				background: linear-gradient(180deg, #ffffff 0%, #d9e9ff 100%);
				font-weight: bold;
				color: #3d3d3d;
				padding: 7px 12px;
				display: flex;
				align-items: center;
				min-width: 470px;
				img {
					width: 20px;
					height: auto;
					margin-right: 10px;
				}
			}
			.select {
				position: absolute;
				right: 14px;
				top: 35px;
				z-index: 10;
			}
			.cbottom {
				width: 98%;
				height: 84%;
				padding: 5px;
				min-width: 470px;
				.graph-box {
					height: 10%;
					display: flex;
					align-items: center;
					padding: 30px 100px;
					justify-content: space-between;
					.box {
						min-width: 188px;
						img {
							width: 47px;
							height: auto;
							margin-right: 10px;
						}
						.text {
							font-size: 16px;
							.title {
								font-weight: 900;
								margin-bottom: 5px;
							}
						}
					}
				}
			}
			.pie {
				display: flex;
				align-items: center;
				justify-content: center;
				.total {
					width: 50%;
					height: 100%;
				}
				.first {
					width: 20%;
					font-size: 12px;
					font-weight: bold;
					color: #767676;
					.seacon {
						margin-bottom: 10px;
						justify-content: flex-start;
						img {
							height: 16px;
							width: 16px;
							margin-right: 5px;
						}
					}
					span {
						margin-right: 5px;
					}
				}
			}
		}
	}
	.map {
		width: 800px;
		height: 600px;
		position: relative;
	}
}
</style>
