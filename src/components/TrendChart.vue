<template>
  <div ref="chartRef" style="width: 100%; height: 280px"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'

const props = defineProps<{
  data: Array<{ hour: string; count: number }>
}>()

const chartRef = ref<HTMLElement | null>(null)
let chart: echarts.ECharts | null = null

onMounted(() => {
  if (chartRef.value) {
    chart = echarts.init(chartRef.value, 'dark')
    updateChart()
  }
})

watch(() => props.data, updateChart, { deep: true })

function updateChart() {
  if (!chart) return
  chart.setOption({
    title: { text: '审批趋势（24小时）', textStyle: { fontSize: 14 } },
    xAxis: { type: 'category', data: props.data.map(d => d.hour) },
    yAxis: { type: 'value' },
    series: [{ type: 'line', data: props.data.map(d => d.count), smooth: true, areaStyle: {} }],
    tooltip: { trigger: 'axis' },
  })
}
</script>
