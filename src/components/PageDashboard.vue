<template>
  <div class="page-dashboard">
    <h2>仪表盘</h2>
    <div class="stats">
      <div class="stat-card">
        <span class="stat-value">128</span>
        <span class="stat-label">待处理审批</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">56</span>
        <span class="stat-label">本月已完成</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">3</span>
        <span class="stat-label">异常流程</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.page-dashboard { padding: 20px; }
.stats { display: flex; gap: 16px; margin-top: 16px; }
.stat-card { padding: 20px; background: #f0f5ff; border-radius: 8px; text-align: center; flex: 1; }
.stat-value { display: block; font-size: 28px; font-weight: bold; color: #1890ff; }
.stat-label { display: block; margin-top: 8px; color: #666; font-size: 14px; }
</style>
