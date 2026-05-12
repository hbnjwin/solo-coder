<template>
  <div ref="chartRef" style="width: 100%; height: 280px"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'

const props = defineProps<{
  data: Array<Array<number | string>>
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
  const hours = Array.from({ length: 24 }, (_, i) => `${i}:00`)
  const depts = [...new Set(props.data.map(d => d[1] as string))]
  chart.setOption({
    title: { text: '审批耗时分布热力图', textStyle: { fontSize: 14 } },
    xAxis: { type: 'category', data: hours },
    yAxis: { type: 'category', data: depts },
    visualMap: { min: 0, max: 120, calculable: true, orient: 'horizontal', left: 'center', bottom: 0 },
    series: [{ type: 'heatmap', data: props.data, emphasis: { itemStyle: { shadowBlur: 10, shadowColor: 'rgba(0, 0, 0, 0.5)' } } }],
    tooltip: { position: 'top' },
    grid: { top: 40, bottom: 60 },
  })
}
</script>
