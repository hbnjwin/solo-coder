<template>
  <div v-if="visible" class="submit-result" :class="success ? 'success' : 'failure'">
    <h3>{{ success ? '提交成功' : '提交失败' }}</h3>
    <p v-if="success">审批单已提交，审批人: {{ approverLabel }}</p>
    <p v-else>请检查表单填写是否完整</p>
    <button @click="$emit('close')" class="close-btn">关闭</button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  visible: boolean
  success: boolean
  approverLabel: string
}>()

defineEmits<{
  close: []
}>()
</script>

<style scoped>
.submit-result { padding: 16px; border-radius: 6px; margin-top: 20px; }
.success { background: #f6ffed; border: 1px solid #b7eb8f; }
.failure { background: #fff2f0; border: 1px solid #ffccc7; }
.close-btn { margin-top: 10px; padding: 4px 12px; border: 1px solid #d9d9d9; border-radius: 4px; background: white; cursor: pointer; }
</style>
