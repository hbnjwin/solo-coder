<template>
  <span class="escalation-badge" :class="badgeClass">{{ badgeText }}</span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { EscalationItem } from '../types'

const props = defineProps<{
  item: EscalationItem
}>()

const badgeClass = computed(() => {
  if (isOverdue.value) return 'badge-overdue'
  if (props.item.status === 'escalated') return 'badge-escalated'
  return 'badge-normal'
})

const isOverdue = computed(() => {
  return props.item.escalationLevel > 0
})

const badgeText = computed(() => {
  if (isOverdue.value) return '已超时'
  if (props.item.status === 'escalated') return '已升级'
  if (props.item.status === 'approved') return '已通过'
  if (props.item.status === 'rejected') return '已拒绝'
  return '待审批'
})
</script>

<style scoped>
.escalation-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}
.badge-overdue {
  background: #fff1f0;
  color: #cf1322;
  border: 1px solid #ffa39e;
}
.badge-escalated {
  background: #fffbe6;
  color: #ad6800;
  border: 1px solid #ffe58f;
}
.badge-normal {
  background: #f6ffed;
  color: #389e0d;
  border: 1px solid #b7eb8f;
}
</style>
