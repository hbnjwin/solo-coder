<template>
  <div class="notification-list">
    <!-- FIXME: virtual scrolling needed for large lists -->
    <div class="list-header">
      <h3>通知中心</h3>
      <button class="mark-all-btn" @click="$emit('mark-all-read')">全部已读</button>
    </div>
    <div class="list-body">
      <NotificationItem
        v-for="n in notifications"
        :key="n.id"
        :notification="n"
        @mark-read="$emit('mark-read', $event)"
      />
      <div v-if="notifications.length === 0" class="empty-state">
        暂无通知
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Notification } from '../types'
import NotificationItem from './NotificationItem.vue'

defineProps<{ notifications: Notification[] }>()
defineEmits<{ 'mark-read': [id: string]; 'mark-all-read': [] }>()
</script>

<style scoped>
.notification-list {
  position: absolute;
  top: 56px;
  right: 16px;
  width: 360px;
  max-height: 480px;
  background: white;
  border: 1px solid #e8e8e8;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  z-index: 100;
}
.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
}
.list-header h3 {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: #1a1a1a;
}
.mark-all-btn {
  background: none;
  border: none;
  color: #1890ff;
  font-size: 13px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
}
.mark-all-btn:hover {
  background: #e6f7ff;
}
.list-body {
  overflow-y: auto;
  max-height: 420px;
}
.empty-state {
  padding: 40px 16px;
  text-align: center;
  color: #999;
  font-size: 14px;
}
</style>
