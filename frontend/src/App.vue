<template>
  <div class="app-container">
    <header class="app-header">
      <h1>合同审批系统</h1>
      <p class="subtitle">Contract Approval Management</p>
    </header>
    <main>
      <ContractForm @submit="handleSubmit" :loading="loading" />
      <div v-if="result" class="result-panel">
        <h3>审批结果</h3>
        <div class="result-status" :class="result.status">
          {{ statusLabel }}
        </div>
        <dl class="result-details">
          <dt>请求编号</dt>
          <dd>{{ result.requestId }}</dd>
          <dt>优先级</dt>
          <dd>{{ result.priority }}</dd>
          <dt v-if="result.validationErrors.length">错误</dt>
          <dd v-if="result.validationErrors.length" class="errors">
            <ul>
              <li v-for="err in result.validationErrors" :key="err">{{ err }}</li>
            </ul>
          </dd>
        </dl>
      </div>
      <div v-if="error" class="error-panel">
        <p>{{ error }}</p>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import ContractForm from './components/ContractForm.vue'
import { submitApproval } from './api'
import type { ApprovalResponse, ContractApprovalPayload } from './types'

const loading = ref(false)
const result = ref<ApprovalResponse | null>(null)
const error = ref<string | null>(null)

const statusLabel = computed(() => {
  if (!result.value) return ''
  const labels: Record<string, string> = {
    accepted: '已通过',
    rejected: '已拒绝',
    error: '处理错误',
  }
  return labels[result.value.status] || result.value.status
})

async function handleSubmit(payload: ContractApprovalPayload) {
  loading.value = true
  error.value = null
  result.value = null

  try {
    result.value = await submitApproval(payload)
  } catch (e: unknown) {
    error.value = e instanceof Error ? e.message : '未知错误'
  } finally {
    loading.value = false
  }
}
</script>

<style>
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #f5f7fa; }
.app-container { max-width: 720px; margin: 0 auto; padding: 32px 16px; }
.app-header { text-align: center; margin-bottom: 32px; }
.app-header h1 { font-size: 24px; color: #1a1a1a; }
.subtitle { color: #666; font-size: 14px; margin-top: 4px; }
.result-panel { margin-top: 24px; padding: 20px; background: white; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.result-panel h3 { margin-bottom: 12px; }
.result-status { display: inline-block; padding: 4px 12px; border-radius: 4px; font-weight: 500; }
.result-status.accepted { background: #e6f7e6; color: #1a7a1a; }
.result-status.rejected { background: #fff2e6; color: #a65800; }
.result-status.error { background: #ffe6e6; color: #a61a1a; }
.result-details { margin-top: 16px; }
.result-details dt { font-weight: 600; color: #333; margin-top: 8px; }
.result-details dd { color: #555; margin-left: 0; }
.errors ul { list-style: none; }
.errors li { color: #c00; font-size: 14px; }
.error-panel { margin-top: 24px; padding: 16px; background: #ffe6e6; border-radius: 8px; color: #a61a1a; }
</style>
