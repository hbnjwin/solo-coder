<template>
  <div v-if="isSyncing" class="sync-progress">
    <div class="progress-label">同步中 {{ syncProgress }}/{{ syncTotal }}</div>
    <div class="progress-bar">
      <div class="progress-fill" :style="{ width: percentage + '%' }"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSyncEngine } from '../composables/useSyncEngine'

const { isSyncing, syncProgress, syncTotal } = useSyncEngine()

const percentage = computed(() =>
  syncTotal.value > 0 ? Math.round((syncProgress.value / syncTotal.value) * 100) : 0
)
</script>

<style scoped>
.sync-progress {
  margin-top: 16px;
  padding: 12px 16px;
  background: #fffbe6;
  border: 1px solid #ffe58f;
  border-radius: 6px;
}
.progress-label {
  font-size: 13px;
  color: #ad6800;
  margin-bottom: 8px;
}
.progress-bar {
  height: 6px;
  background: #fff1b8;
  border-radius: 3px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: #faad14;
  border-radius: 3px;
  transition: width 0.3s ease;
}
</style>
