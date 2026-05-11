<template>
  <div class="network-indicator" :class="{ offline: !isOnline }">
    <span class="dot"></span>
    <span class="label">{{ isOnline ? '在线' : '离线' }}</span>
  </div>
</template>

<script setup lang="ts">
import { useNetworkStatus } from '../composables/useNetworkStatus'

const { isOnline } = useNetworkStatus()
</script>

<style scoped>
.network-indicator {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 16px;
  background: #f6ffed;
  border: 1px solid #b7eb8f;
  font-size: 13px;
}
.network-indicator.offline {
  background: #fff2f0;
  border-color: #ffccc7;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #52c41a;
}
.offline .dot {
  background: #ff4d4f;
}
</style>
