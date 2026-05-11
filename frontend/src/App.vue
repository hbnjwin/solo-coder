<template>
  <div class="app-container">
    <h2>大数据导出管理</h2>
    <ExportPanel />
    <ExportHistory />
  </div>
</template>

<script setup lang="ts">
import ExportPanel from './components/ExportPanel.vue'
import ExportHistory from './components/ExportHistory.vue'
</script>

<style>
body { margin: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; }
.app-container { max-width: 640px; margin: 40px auto; padding: 24px; }
h2 { margin-bottom: 24px; }
</style>
