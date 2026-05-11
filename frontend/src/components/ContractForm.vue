<template>
  <form class="contract-form" @submit.prevent="onSubmit">
    <div class="form-group">
      <label for="contractName">合同名称</label>
      <input id="contractName" v-model="form.contractName" type="text" required placeholder="请输入合同名称" />
    </div>

    <div class="form-row">
      <div class="form-group">
        <label for="sheetAmount">金额 (元)</label>
        <input id="sheetAmount" v-model.number="form.sheetAmount" type="number" min="0" step="0.01" required />
      </div>
      <div class="form-group">
        <label for="department">部门</label>
        <input id="department" v-model="form.department" type="text" placeholder="所属部门" />
      </div>
    </div>

    <div class="form-group">
      <label for="approver">审批人</label>
      <input id="approver" v-model="form.approver" type="text" required placeholder="审批人姓名" />
    </div>

    <div class="form-group">
      <label for="submissionDate">提交日期</label>
      <input id="submissionDate" v-model="form.submissionDate" type="datetime-local" required />
    </div>

    <div class="form-group checkbox-group">
      <label>
        <input v-model="form.isUrgent" type="checkbox" />
        <span>紧急审批</span>
      </label>
    </div>

    <div class="form-group">
      <label for="notes">备注</label>
      <textarea id="notes" v-model="form.notes" rows="3" placeholder="可选备注信息"></textarea>
    </div>

    <button type="submit" :disabled="loading" class="submit-btn">
      {{ loading ? '提交中...' : '提交审批' }}
    </button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import type { FormState, ContractApprovalPayload } from '../types'

const props = defineProps<{ loading: boolean }>()
const emit = defineEmits<{ submit: [payload: ContractApprovalPayload] }>()

const form = reactive<FormState>({
  contractName: '',
  sheetAmount: 0,
  approver: '',
  department: '',
  submissionDate: '',
  isUrgent: false,
  notes: '',
})

function onSubmit() {
  const payload: ContractApprovalPayload = {
    contractName: form.contractName,
    sheetAmount: form.sheetAmount,
    approver: form.approver,
    department: form.department,
    submissionDate: form.submissionDate,
    isUrgent: form.isUrgent,
    notes: form.notes,
  }
  emit('submit', payload)
}
</script>

<style scoped>
.contract-form { background: white; padding: 24px; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.form-group { margin-bottom: 16px; }
.form-group label { display: block; font-weight: 500; margin-bottom: 6px; color: #333; font-size: 14px; }
.form-group input[type="text"],
.form-group input[type="number"],
.form-group input[type="datetime-local"],
.form-group textarea { width: 100%; padding: 10px 12px; border: 1px solid #d9d9d9; border-radius: 6px; font-size: 14px; transition: border-color 0.2s; }
.form-group input:focus, .form-group textarea:focus { outline: none; border-color: #1890ff; box-shadow: 0 0 0 2px rgba(24,144,255,0.1); }
.form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
.checkbox-group label { display: flex; align-items: center; gap: 8px; cursor: pointer; }
.checkbox-group input[type="checkbox"] { width: 16px; height: 16px; }
.submit-btn { width: 100%; padding: 12px; background: #1890ff; color: white; border: none; border-radius: 6px; font-size: 16px; font-weight: 500; cursor: pointer; transition: background 0.2s; }
.submit-btn:hover:not(:disabled) { background: #096dd9; }
.submit-btn:disabled { opacity: 0.6; cursor: not-allowed; }
</style>
