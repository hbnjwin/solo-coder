<template>
  <div class="container">
    <h2>合同审批表单</h2>
    <form @submit.prevent="handleSubmit">
      <div class="field"><label>合同名称</label><input v-model="form.contractName" placeholder="请输入合同名称" /></div>
      <div class="field"><label>金额</label><input v-model.number="form.amount" type="number" placeholder="请输入金额" /></div>
      <div class="field"><label>审批人</label>
        <!-- BUG: no "总经理" option even when amount > 100万 -->
        <select v-model="form.approver"><option value="">请选择</option><option value="manager">部门经理</option></select>
      </div>
      <div class="field"><label>日期</label><input v-model="form.date" type="date" /></div>
      <!-- BUG: no validation at all -->
      <button type="submit">提交</button>
    </form>
    <p v-if="submitted">已提交: {{ JSON.stringify(form) }}</p>
  </div>
</template>
<script setup lang="ts">
import { reactive, ref } from 'vue'
const form = reactive({ contractName: '', amount: 0, approver: '', date: '' })
const submitted = ref(false)
function handleSubmit() {
  // BUG: no validation - any data can be submitted
  submitted.value = true
}
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.field { margin-bottom: 16px; }
.field label { display: block; margin-bottom: 4px; font-weight: bold; }
.field input, .field select { width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; }
button { padding: 10px 24px; background: #1890ff; color: white; border: none; border-radius: 4px; cursor: pointer; }
</style>