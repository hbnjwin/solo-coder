<template>
  <div class="dashboard">
    <header class="dashboard-header">
      <h1>审批数据实时监控大屏</h1>
      <span class="clock">{{ currentTime }}</span>
    </header>
    <div class="stats-row">
      <StatCard title="今日审批总量" :value="stats.totalCount" color="#1890ff" />
      <StatCard title="平均审批时长" :value="stats.avgDuration" suffix="min" color="#52c41a" />
      <StatCard title="当前待审批数" :value="stats.pendingCount" color="#faad14" />
      <StatCard title="超时预警数" :value="stats.timeoutCount" color="#ff4d4f" />
    </div>
    <div class="charts-row">
      <div class="chart-container">
        <TrendChart :data="trendData" />
      </div>
      <div class="chart-container">
        <DeptChart :data="deptData" />
      </div>
    </div>
    <div class="charts-row">
      <div class="chart-container">
        <TypePieChart :data="typeData" />
      </div>
      <div class="chart-container">
        <HeatmapChart :data="heatmapData" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import StatCard from './components/StatCard.vue'
import TrendChart from './components/TrendChart.vue'
import DeptChart from './components/DeptChart.vue'
import TypePieChart from './components/TypePieChart.vue'
import HeatmapChart from './components/HeatmapChart.vue'
import { useMockData } from './composables/useMockData'

const { stats, trendData, deptData, typeData, heatmapData, refresh } = useMockData()

const currentTime = ref('')
let timer: ReturnType<typeof setInterval> | null = null
let refreshTimer: ReturnType<typeof setInterval> | null = null

function updateClock() {
  currentTime.value = new Date().toLocaleString('zh-CN')
}

onMounted(() => {
  updateClock()
  timer = setInterval(updateClock, 1000)
  refreshTimer = setInterval(refresh, 5000)
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #0d1b2a;
  color: #e0e0e0;
}
.dashboard {
  min-height: 100vh;
  padding: 16px 24px;
}
.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
.dashboard-header h1 {
  font-size: 22px;
  color: #fff;
}
.clock {
  font-size: 16px;
  color: #8ecae6;
}
.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 20px;
}
.charts-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
}
.chart-container {
  background: #1b2838;
  border-radius: 8px;
  padding: 16px;
  min-height: 300px;
}
</style>
