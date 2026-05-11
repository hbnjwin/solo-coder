<template>
  <header class="header-bar">
    <div class="header-left">
      <h1 class="app-title">工作台</h1>
    </div>
    <div class="header-right">
      <NotificationBadge :count="unreadCount" @toggle="$emit('toggle-panel')" />
    </div>
  </header>
</template>

<script setup lang="ts">
import NotificationBadge from './NotificationBadge.vue'

defineProps<{ unreadCount: number }>()
defineEmits<{ 'toggle-panel': [] }>()
</script>

<style scoped>
.header-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 24px;
  background: white;
  border-bottom: 1px solid #e8e8e8;
  position: relative;
}
.app-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1a1a1a;
}
.header-right {
  display: flex;
  align-items: center;
}
</style>
