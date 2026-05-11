<template>
  <div class="spinner-wrapper" v-if="visible">
    <div class="spinner"></div>
    <span class="spinner-text">{{ text }}</span>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  visible: boolean
  text?: string
}>(), {
  text: '加载中...',
})
</script>

<style scoped>
.spinner-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 0;
  color: #666;
}
.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #e0e0e0;
  border-top-color: #1890ff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
.spinner-text {
  font-size: 13px;
}
</style>
