<template>
  <div class="notification-item" :class="{ unread: !notification.read }" @click="$emit('mark-read', notification.id)">
    <div class="item-header">
      <span class="priority-dot" :class="notification.priority"></span>
      <span class="item-title">{{ notification.title }}</span>
    </div>
    <p class="item-body">{{ notification.body }}</p>
    <span class="item-time">{{ formattedTime }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Notification } from '../types'

const props = defineProps<{ notification: Notification }>()
defineEmits<{ 'mark-read': [id: string] }>()

const formattedTime = computed(() => {
  const diff = Date.now() - props.notification.timestamp
  const minutes = Math.floor(diff / 60000)
  if (minutes < 60) return `${minutes}分钟前`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}小时前`
  return `${Math.floor(hours / 24)}天前`
})
</script>

<style scoped>
.notification-item {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background 0.15s;
}
.notification-item:hover {
  background: #fafafa;
}
.notification-item.unread {
  background: #e8f4fd;
}
.notification-item.unread:hover {
  background: #d6ecf8;
}
.item-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}
.priority-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}
.priority-dot.high { background: #e53935; }
.priority-dot.medium { background: #fb8c00; }
.priority-dot.low { background: #43a047; }
.item-title {
  font-size: 14px;
  font-weight: 500;
  color: #1a1a1a;
}
.item-body {
  font-size: 13px;
  color: #666;
  margin: 2px 0 6px 16px;
}
.item-time {
  font-size: 12px;
  color: #999;
  margin-left: 16px;
}
</style>
