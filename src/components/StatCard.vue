<template>
  <div class="stat-card" :style="{ borderTopColor: color }">
    <div class="stat-title">{{ title }}</div>
    <div class="stat-value">
      <span class="stat-number">{{ displayValue }}</span>
      <span v-if="suffix" class="stat-suffix">{{ suffix }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  title: string
  value: number
  suffix?: string
  color: string
}>()

const displayValue = ref(0)

watch(() => props.value, (newVal, oldVal) => {
  const start = oldVal ?? 0
  const diff = newVal - start
  const duration = 800
  const startTime = Date.now()
  function animate() {
    const elapsed = Date.now() - startTime
    const progress = Math.min(elapsed / duration, 1)
    const eased = 1 - Math.pow(1 - progress, 3)
    displayValue.value = Math.round(start + diff * eased)
    if (progress < 1) {
      requestAnimationFrame(animate)
    }
  }
  animate()
}, { immediate: true })
</script>

<style scoped>
.stat-card {
  background: #1b2838;
  border-radius: 8px;
  padding: 20px;
  border-top: 3px solid;
}
.stat-title {
  font-size: 13px;
  color: #8ecae6;
  margin-bottom: 8px;
}
.stat-value {
  display: flex;
  align-items: baseline;
  gap: 4px;
}
.stat-number {
  font-size: 32px;
  font-weight: 700;
  color: #fff;
}
.stat-suffix {
  font-size: 14px;
  color: #8ecae6;
}
</style>
