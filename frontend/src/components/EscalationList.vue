<template>
  <div class="escalation-list">
    <div
      v-for="item in items"
      :key="item.id"
      class="escalation-item"
      :class="{ 'item-resolved': item.resolvedAt !== null }"
    >
      <div class="item-header">
        <span class="item-title">{{ item.title }}</span>
        <EscalationBadge :item="item" />
      </div>
      <div class="item-meta">
        <span>审批人: {{ item.approver }}</span>
        <span>提交时间: {{ formatDate(item.submittedAt) }}</span>
        <span v-if="item.escalationLevel > 0">升级次数: {{ item.escalationLevel }}</span>
      </div>
    </div>
    <div v-if="items.length === 0" class="empty-state">暂无审批记录</div>
  </div>
</template>

<script setup lang="ts">
import type { EscalationItem } from '../types'
import EscalationBadge from './EscalationBadge.vue'

defineProps<{
  items: EscalationItem[]
}>()

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
}
</script>

<style scoped>
.escalation-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.escalation-item {
  padding: 16px;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  transition: border-color 0.2s;
}
.escalation-item:hover {
  border-color: #1890ff;
}
.item-resolved {
  opacity: 0.6;
}
.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}
.item-title {
  font-weight: 600;
  font-size: 14px;
}
.item-meta {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: #8c8c8c;
}
.empty-state {
  text-align: center;
  padding: 40px;
  color: #bfbfbf;
}
</style>
