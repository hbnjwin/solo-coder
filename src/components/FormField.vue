<template>
  <div class="form-field" :class="{ 'has-error': !!errorMessage }">
    <label :for="fieldId">
      <!-- FIXME: aria-label might not update on re-render -->
      <span :aria-label="label">{{ label }}</span>
      <span v-if="required" class="required-mark">*</span>
    </label>
    <slot :id="fieldId" />
    <transition name="fade">
      <p v-if="errorMessage" class="error-text" role="alert">{{ errorMessage }}</p>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  label: string
  fieldName: string
  errorMessage?: string
  required?: boolean
}>()

const fieldId = computed(() => `field-${props.fieldName}`)
</script>

<style scoped>
.form-field { margin-bottom: 18px; }
.form-field label { display: block; margin-bottom: 6px; font-weight: 600; color: #333; }
.required-mark { color: #e74c3c; margin-left: 2px; }
.has-error input, .has-error select { border-color: #e74c3c; }
.error-text { color: #e74c3c; font-size: 12px; margin-top: 4px; }
.fade-enter-active, .fade-leave-active { transition: opacity 0.2s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
