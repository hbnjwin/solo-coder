<template>
  <div class="export-history">
    <h4>导出历史</h4>
    <div v-if="history.length === 0" class="empty">暂无导出记录</div>
    <ul v-else>
      <li v-for="entry in history" :key="entry.id" class="history-item">
        <span class="history-time">{{ formatTime(entry.startedAt) }}</span>
        <span class="history-count">{{ entry.recordCount }} 条</span>
        <span class="history-status" :class="entry.status">{{ entry.status }}</span>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ExportHistoryEntry } from '../types'

const history = ref<ExportHistoryEntry[]>([])

function formatTime(ts: number): string {
  return new Date(ts).toLocaleString('zh-CN')
}
</script>

<style scoped>
.export-history { margin-top: 24px; }
.export-history h4 { margin-bottom: 12px; }
.empty { color: #999; font-size: 13px; }
ul { list-style: none; padding: 0; }
.history-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
  font-size: 13px;
}
.history-status.complete { color: #52c41a; }
.history-status.failed { color: #ff4d4f; }
.history-status.cancelled { color: #faad14; }
</style>
