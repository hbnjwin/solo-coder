<template>
  <div class="approver-select">
    <select
      :id="id"
      :value="modelValue"
      @change="handleChange"
      :disabled="options.length === 0"
    >
      <option value="">请选择审批人</option>
      <option
        v-for="option in options"
        :key="option.value"
        :value="option.value"
      >
        {{ option.label }}
      </option>
    </select>
    <p v-if="recommended" class="recommendation">
      建议审批人: {{ recommendedLabel }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ApproverOption } from '../types'

const props = defineProps<{
  id?: string
  modelValue: string
  options: ApproverOption[]
  recommended?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

function handleChange(event: Event) {
  const target = event.target as HTMLSelectElement
  emit('update:modelValue', target.value)
}

const recommendedLabel = computed(() => {
  return props.options.find(o => o.value === props.recommended)?.label ?? ''
})
</script>

<style scoped>
.approver-select select { width: 100%; padding: 8px 12px; border: 1px solid #d9d9d9; border-radius: 4px; font-size: 14px; }
.recommendation { font-size: 12px; color: #1890ff; margin-top: 4px; }
</style>
