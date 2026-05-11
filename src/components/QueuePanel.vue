<template>
  <div v-if="queue.length > 0" class="queue-panel">
    <h3 class="queue-title">同步队列 ({{ queue.length }})</h3>
    <div class="queue-list">
      <div
        v-for="item in queue"
        :key="item.id"
        class="queue-item"
        :class="item.status"
      >
        <div class="item-info">
          <span class="contract">{{ item.payload.contractId }}</span>
          <span class="status-badge">{{ statusLabel(item.status) }}</span>
        </div>
        <div class="item-meta">
          {{ item.payload.approver }} · {{ formatTime(item.timestamp) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useOfflineQueue } from '../composables/useOfflineQueue'
import type { SyncStatus } from '../types'

const { queue } = useOfflineQueue()

function statusLabel(status: SyncStatus): string {
  const labels: Record<SyncStatus, string> = {
    pending: '待同步',
    syncing: '同步中',
    synced: '已同步',
    failed: '失败'
  }
  return labels[status]
}

function formatTime(ts: number): string {
  return new Date(ts).toLocaleTimeString()
}
</script>

<style scoped>
.queue-panel {
  margin-top: 24px;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  overflow: hidden;
}
.queue-title {
  margin: 0;
  padding: 12px 16px;
  background: #fafafa;
  border-bottom: 1px solid #e8e8e8;
  font-size: 14px;
}
.queue-list {
  max-height: 300px;
  overflow-y: auto;
}
.queue-item {
  padding: 10px 16px;
  border-bottom: 1px solid #f0f0f0;
}
.queue-item:last-child {
  border-bottom: none;
}
.item-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.contract {
  font-weight: 500;
}
.status-badge {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 10px;
  background: #e6f4ff;
  color: #1677ff;
}
.queue-item.synced .status-badge {
  background: #f6ffed;
  color: #52c41a;
}
.queue-item.failed .status-badge {
  background: #fff2f0;
  color: #ff4d4f;
}
.queue-item.syncing .status-badge {
  background: #fffbe6;
  color: #faad14;
}
.item-meta {
  margin-top: 4px;
  font-size: 12px;
  color: #999;
}
</style>
