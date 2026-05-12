<template>
  <div ref="chartRef" style="width: 100%; height: 280px"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'

const props = defineProps<{
  data: Array<{ dept: string; rate: number }>
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
    title: { text: '各部门审批通过率', textStyle: { fontSize: 14 } },
    xAxis: { type: 'category', data: props.data.map(d => d.dept) },
    yAxis: { type: 'value', max: 100 },
    series: [{ type: 'bar', data: props.data.map(d => d.rate) }],
    tooltip: { trigger: 'axis' },
  })
}
</script>
