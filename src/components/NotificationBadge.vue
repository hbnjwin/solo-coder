<template>
  <div class="badge-wrapper" @click="$emit('toggle')">
    <span class="bell-icon">🔔</span>
    <span v-if="count > 0" class="badge">{{ count }}</span>
  </div>
</template>

<script setup lang="ts">
defineProps<{ count: number }>()
defineEmits<{ toggle: [] }>()
</script>

<style scoped>
.badge-wrapper {
  position: relative;
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  font-size: 22px;
  padding: 4px 8px;
  border-radius: 6px;
  transition: background 0.2s;
}
.badge-wrapper:hover {
  background: rgba(0, 0, 0, 0.05);
}
.badge {
  position: absolute;
  top: -4px;
  right: -2px;
  background: #e53935;
  color: white;
  border-radius: 10px;
  padding: 1px 6px;
  font-size: 11px;
  font-weight: 600;
  min-width: 16px;
  text-align: center;
  line-height: 16px;
}
</style>
