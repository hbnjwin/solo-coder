<template>
  <div ref="chartRef" style="width: 100%; height: 280px"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'

const props = defineProps<{
  data: Array<{ name: string; value: number }>
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
    title: { text: '审批类型分布', textStyle: { fontSize: 14 } },
    series: [{ type: 'pie', radius: ['40%', '70%'], data: props.data }],
    tooltip: { trigger: 'item' },
  })
}
</script>
