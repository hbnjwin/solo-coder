<template>
  <div class="container">
    <h2>审批客户端</h2>
    <div class="status" :class="{ offline: !isOnline }">
      {{ isOnline ? '在线' : '离线' }}
    </div>
    <form @submit.prevent="submitApproval">
      <div class="field"><label>合同编号</label><input v-model="form.contractId" /></div>
      <div class="field"><label>审批意见</label><input v-model="form.comment" /></div>
      <button type="submit">提交审批</button>
    </form>
    <div v-if="pendingQueue.length > 0" class="queue">
      <h3>待同步队列 ({{ pendingQueue.length }})</h3>
      <div v-for="(item, idx) in pendingQueue" :key="idx" class="queue-item">
        {{ item.contractId }} - {{ item.comment }} <span class="time">{{ item.timestamp }}</span>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { reactive, ref } from 'vue'
const isOnline = ref(navigator.onLine)
const form = reactive({ contractId: '', comment: '' })
const pendingQueue = reactive<{ contractId: string; comment: string; timestamp: number }[]>([])

// BUG: fails completely when offline, no queue mechanism
// BUG: no queue limit - can grow unbounded
// BUG: sync order not guaranteed (would need timestamp-based sorting)
function submitApproval() {
  if (!isOnline.value) {
    alert('网络不可用，提交失败') // BUG: should queue instead
    return
  }
  console.log('submitted:', form)
}
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.status { padding: 8px 16px; background: #52c41a; color: white; border-radius: 4px; display: inline-block; margin-bottom: 16px; }
.status.offline { background: #ff4d4f; }
.field { margin-bottom: 16px; }
.field label { display: block; margin-bottom: 4px; font-weight: bold; }
.field input { width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; }
.queue { margin-top: 20px; padding: 16px; border: 1px solid #faad14; border-radius: 4px; background: #fffbe6; }
.queue-item { padding: 8px; border-bottom: 1px solid #ffe58f; }
.time { color: #999; font-size: 12px; margin-left: 8px; }
</style>