<template>
  <div class="app-container">
    <HeaderBar :unread-count="unreadCount" @toggle-panel="isPanelOpen = !isPanelOpen" />
    <NotificationList
      v-if="isPanelOpen"
      :notifications="notifications"
      @mark-read="markRead"
      @mark-all-read="markAllRead"
    />
    <main class="main-content">
      <p class="welcome-text">欢迎使用工作台，您有 <strong>{{ unreadCount }}</strong> 条未读通知</p>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import HeaderBar from './components/HeaderBar.vue'
import NotificationList from './components/NotificationList.vue'
import { useNotifications } from './composables/useNotifications'
import { startPushSimulation } from './services/pushSimulator'

const isPanelOpen = ref(false)
const { notifications, unreadCount, markRead, markAllRead, addNotification } = useNotifications()

onMounted(() => {
  startPushSimulation((notification) => {
    addNotification(notification)
  })
})
</script>

<style>
* {
  box-sizing: border-box;
}
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
  color: #333;
}
.app-container {
  position: relative;
  min-height: 100vh;
}
.main-content {
  padding: 32px 24px;
}
.welcome-text {
  font-size: 15px;
  color: #555;
}
</style>
