<template>
  <div class="export-panel">
    <div class="panel-header">
      <h3>数据导出</h3>
      <span class="status-badge" :class="state.status">{{ statusLabel }}</span>
    </div>

    <div class="progress-section" v-if="state.status !== 'idle'">
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: progress + '%' }"></div>
      </div>
      <div class="progress-info">
        <span>{{ state.pagesCompleted }} / {{ state.totalPages }} 页</span>
        <span>{{ progress }}%</span>
      </div>
    </div>

    <div class="error-message" v-if="state.errorMessage">
      {{ state.errorMessage }}
    </div>

    <div class="actions">
      <button @click="startExport" :disabled="state.status === 'exporting'">
        开始导出
      </button>
      <button @click="pause" v-if="state.status === 'exporting'">
        暂停
      </button>
      <button @click="resume" v-if="state.status === 'error' || state.status === 'paused'">
        继续
      </button>
      <button @click="reset" v-if="state.status === 'complete' || state.status === 'error'">
        重置
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useExport } from '../composables/useExport'

const { state, progress, startExport, pause, resume, reset } = useExport()

const statusLabel = computed(() => {
  const labels: Record<string, string> = {
    idle: '就绪',
    exporting: '导出中',
    paused: '已暂停',
    error: '错误',
    complete: '完成',
  }
  return labels[state.value.status] || state.value.status
})
</script>

<style scoped>
.export-panel {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 16px;
}
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.panel-header h3 { margin: 0; }
.status-badge {
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
}
.status-badge.exporting { background: #e6f7ff; color: #1890ff; }
.status-badge.complete { background: #f6ffed; color: #52c41a; }
.status-badge.error { background: #fff2f0; color: #ff4d4f; }
.status-badge.paused { background: #fffbe6; color: #faad14; }
.progress-section { margin: 16px 0; }
.progress-bar {
  height: 8px;
  background: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: #1890ff;
  border-radius: 4px;
  transition: width 0.3s ease;
}
.progress-info {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 13px;
  color: #666;
}
.error-message {
  padding: 8px 12px;
  background: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
  color: #ff4d4f;
  margin: 12px 0;
  font-size: 13px;
}
.actions { display: flex; gap: 8px; margin-top: 16px; }
.actions button {
  padding: 8px 16px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  cursor: pointer;
  background: #fff;
}
.actions button:hover:not(:disabled) { border-color: #1890ff; color: #1890ff; }
.actions button:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
