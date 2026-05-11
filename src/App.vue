<template>
  <div class="container">
    <h2>消息通知</h2>
    <div class="bell" @click="showList = !showList">
      🔔 <span v-if="unreadCount > 0" class="badge">{{ unreadCount }}</span>
    </div>
    <div v-if="showList" class="notification-list">
      <div v-for="n in notifications" :key="n.id" class="item" :class="{ unread: !n.read }" @click="markRead(n.id)">
        <div class="title">{{ n.title }}</div>
        <div class="meta">{{ n.type }} · {{ n.time }}</div>
      </div>
      <button @click="clearAll">清空</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
interface Notif { id: number; title: string; type: string; time: string; read: boolean }
const showList = ref(false)
const notifications = ref<Notif[]>([
  { id: 1, title: '合同A等待审批', type: '审批', time: '10:00', read: false },
  { id: 2, title: '系统维护通知', type: '系统', time: '09:30', read: false },
  { id: 3, title: '合同B已通过', type: '审批', time: '昨天', read: true },
])
const unreadCount = computed(() => notifications.value.filter(n => !n.read).length)
function markRead(id: number) { const n = notifications.value.find(n => n.id === id); if (n) n.read = true }
function clearAll() { notifications.value = [] }
</script>
<style>
.container { padding: 20px; position: relative; }
.bell { font-size: 24px; cursor: pointer; display: inline-block; position: relative; }
.badge { position: absolute; top: -8px; right: -8px; background: red; color: white; border-radius: 50%; padding: 2px 6px; font-size: 12px; }
.notification-list { position: absolute; top: 60px; right: 20px; width: 300px; border: 1px solid #ddd; background: white; border-radius: 4px; }
.item { padding: 10px; border-bottom: 1px solid #eee; cursor: pointer; }
.item.unread { background: #e6f7ff; }
</style>