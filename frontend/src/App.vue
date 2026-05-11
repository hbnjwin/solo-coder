<template>
  <div class="container">
    <h2>审批列表</h2>
    <div v-for="task in tasks" :key="task.id" class="card" :class="{ escalated: task.escalated }">
      <div class="header">{{ task.id }} - {{ task.approver }}</div>
      <div class="status">{{ task.status }}</div>
      <!-- TODO: show escalation badge and reason -->
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const tasks = ref([
  { id: 't1', approver: 'zhangsan', status: 'pending', escalated: false },
  { id: 't2', approver: 'lisi', status: 'approved', escalated: false },
])
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.card { padding: 16px; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 12px; }
.card.escalated { border-color: #faad14; background: #fffbe6; }
.header { font-weight: bold; }
.status { color: #666; margin-top: 4px; }
</style>