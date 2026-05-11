<template>
  <div class="container">
    <h2>审批提交</h2>
    <form @submit.prevent="submit">
      <div class="field"><label>合同名称</label><input v-model="form.contractName" /></div>
      <div class="field"><label>金额</label><input v-model.number="form.sheetAmount" type="number" /></div>
      <div class="field"><label>审批人</label><input v-model="form.approver" /></div>
      <div class="field"><label>日期</label><input v-model="form.date" type="datetime-local" /></div>
      <div class="field"><label>紧急</label><input v-model="form.urgent" type="checkbox" /></div>
      <button type="submit">提交</button>
    </form>
    <!-- BUG: sends camelCase, backend expects snake_case -->
    <!-- BUG: sends ISO date, backend expects YYYY-MM-DD -->
    <!-- BUG: sends boolean, backend expects string -->
  </div>
</template>
<script setup lang="ts">
import { reactive } from 'vue'
const form = reactive({ contractName: '', sheetAmount: 0, approver: '', date: '', urgent: false })
async function submit() {
  // BUG: no field name transformation
  const resp = await fetch('/api/approval', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(form) })
  console.log(await resp.json())
}
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.field { margin-bottom: 16px; }
.field label { display: block; margin-bottom: 4px; font-weight: bold; }
.field input { width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; }
button { padding: 10px 24px; background: #1890ff; color: white; border: none; border-radius: 4px; cursor: pointer; }
</style>