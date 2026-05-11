<template>
  <div class="approval-form-container">
    <h2>合同审批表单</h2>
    <form @submit.prevent="handleSubmit" novalidate>
      <FormField
        label="合同名称"
        field-name="contractName"
        :error-message="getFieldError('contractName')"
        required
      >
        <input
          id="field-contractName"
          v-model="form.contractName"
          type="text"
          placeholder="请输入合同名称"
          @blur="debouncedValidate"
        />
      </FormField>

      <FormField
        label="金额 (元)"
        field-name="amount"
        :error-message="getFieldError('amount')"
        required
      >
        <input
          id="field-amount"
          v-model.number="form.amount"
          type="number"
          placeholder="请输入金额"
          min="0"
          @blur="debouncedValidate"
        />
      </FormField>

      <FormField
        label="审批人"
        field-name="approver"
        :error-message="getFieldError('approver')"
        required
      >
        <ApproverSelect
          id="field-approver"
          v-model="form.approver"
          :options="availableApprovers"
          :recommended="recommendedApprover"
        />
      </FormField>

      <FormField
        label="审批日期"
        field-name="date"
        :error-message="getFieldError('date')"
        required
      >
        <input
          id="field-date"
          v-model="form.date"
          type="date"
          @blur="debouncedValidate"
        />
      </FormField>

      <div class="form-actions">
        <button type="submit" class="submit-btn" :disabled="isValidating">
          {{ isValidating ? '验证中...' : '提交审批' }}
        </button>
        <button type="button" class="reset-btn" @click="resetForm">重置</button>
      </div>
    </form>

    <SubmitResult
      :visible="showResult"
      :success="submitSuccess"
      :approver-label="getApproverLabel(form.approver)"
      @close="showResult = false"
    />
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import type { ApprovalForm } from '../types'
import { useFormValidation } from '../composables/useFormValidation'
import { useApproverRouting } from '../composables/useApproverRouting'
import FormField from './FormField.vue'
import ApproverSelect from './ApproverSelect.vue'
import SubmitResult from './SubmitResult.vue'

const form = reactive<ApprovalForm>({
  contractName: '',
  amount: 0,
  approver: '',
  date: '',
})

const showResult = ref(false)
const submitSuccess = ref(false)

const { errors, isValidating, validate, debouncedValidate, getFieldError, clearErrors } =
  useFormValidation(form)

const { availableApprovers, recommendedApprover, getApproverLabel } =
  useApproverRouting(form)

function handleSubmit() {
  const isValid = validate()
  showResult.value = true
  submitSuccess.value = isValid
}

function resetForm() {
  form.contractName = ''
  form.amount = 0
  form.approver = ''
  form.date = ''
  clearErrors()
  showResult.value = false
}
</script>

<style scoped>
.approval-form-container { max-width: 560px; margin: 0 auto; padding: 24px; }
h2 { margin-bottom: 24px; color: #1a1a1a; font-size: 20px; }
.form-actions { display: flex; gap: 12px; margin-top: 24px; }
.submit-btn { padding: 10px 28px; background: #1890ff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; }
.submit-btn:disabled { background: #91caff; cursor: not-allowed; }
.submit-btn:hover:not(:disabled) { background: #096dd9; }
.reset-btn { padding: 10px 28px; background: white; color: #333; border: 1px solid #d9d9d9; border-radius: 4px; cursor: pointer; font-size: 14px; }
.reset-btn:hover { border-color: #1890ff; color: #1890ff; }
input[type="text"], input[type="number"], input[type="date"] { width: 100%; padding: 8px 12px; border: 1px solid #d9d9d9; border-radius: 4px; font-size: 14px; box-sizing: border-box; }
</style>
