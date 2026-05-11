<template>
  <form class="approval-form" @submit.prevent="handleSubmit">
    <div class="form-field">
      <label for="contractId">合同编号</label>
      <input id="contractId" v-model="form.contractId" placeholder="请输入合同编号" required />
    </div>
    <div class="form-field">
      <label for="approver">审批人</label>
      <input id="approver" v-model="form.approver" placeholder="请输入审批人姓名" required />
    </div>
    <div class="form-field">
      <label for="comment">审批意见</label>
      <textarea id="comment" v-model="form.comment" placeholder="请输入审批意见" rows="3" required></textarea>
    </div>
    <button type="submit" class="submit-btn">提交审批</button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import { useOfflineQueue } from '../composables/useOfflineQueue'
import { useNetworkStatus } from '../composables/useNetworkStatus'
import { useSyncEngine } from '../composables/useSyncEngine'

const { enqueue } = useOfflineQueue()
const { isOnline } = useNetworkStatus()
const { syncAll } = useSyncEngine()

const form = reactive({
  contractId: '',
  comment: '',
  approver: ''
})

function handleSubmit() {
  enqueue({
    contractId: form.contractId,
    comment: form.comment,
    approver: form.approver
  })

  form.contractId = ''
  form.comment = ''
  form.approver = ''

  if (isOnline.value) {
    syncAll()
  }
}
</script>

<style scoped>
.approval-form {
  margin-top: 20px;
}
.form-field {
  margin-bottom: 14px;
}
.form-field label {
  display: block;
  margin-bottom: 4px;
  font-weight: 600;
  font-size: 14px;
  color: #333;
}
.form-field input,
.form-field textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}
.form-field textarea {
  resize: vertical;
}
.submit-btn {
  width: 100%;
  padding: 10px;
  background: #1677ff;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 15px;
  cursor: pointer;
}
.submit-btn:hover {
  background: #4096ff;
}
</style>
