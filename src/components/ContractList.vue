<template>
  <div class="contract-list">
    <h3>合同列表</h3>
    <LoadingSpinner :visible="loading" text="加载合同..." />
    <table v-if="!loading && contracts.length">
      <thead>
        <tr>
          <th>编号</th>
          <th>名称</th>
          <th>金额</th>
          <th>状态</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(contract, index) in contracts"
          :key="index"
          :class="{ selected: selectedId === contract.id }"
          @click="$emit('select', contract.id)"
        >
          <td>{{ contract.id }}</td>
          <td>{{ contract.name }}</td>
          <td>{{ formatAmount(contract.amount) }}</td>
          <td>
            <span :class="['status-tag', statusClass(contract.status)]">
              {{ contract.status }}
            </span>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-if="!loading && !contracts.length" class="empty">暂无合同数据</p>
  </div>
</template>

<script setup lang="ts">
import type { Contract } from '../types'
import LoadingSpinner from './LoadingSpinner.vue'

defineProps<{
  contracts: Contract[]
  selectedId: string
  loading: boolean
}>()

defineEmits<{
  select: [id: string]
}>()

function formatAmount(amount: number): string {
  return `¥${amount.toLocaleString()}`
}

function statusClass(status: string): string {
  switch (status) {
    case '执行中': return 'status-active'
    case '已完成': return 'status-done'
    case '已终止': return 'status-cancelled'
    default: return ''
  }
}
</script>

<style scoped>
.contract-list {
  flex: 1;
  min-width: 0;
}
h3 {
  margin: 0 0 12px;
  font-size: 15px;
  color: #333;
}
table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}
th, td {
  padding: 8px 10px;
  border: 1px solid #e8e8e8;
  text-align: left;
}
th {
  background: #fafafa;
  font-weight: 500;
  color: #555;
}
tr {
  cursor: pointer;
  transition: background 0.15s;
}
tr:hover {
  background: #f5f5f5;
}
tr.selected {
  background: #e6f7ff;
}
.status-tag {
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 12px;
}
.status-active { background: #e6f7ff; color: #1890ff; }
.status-done { background: #f6ffed; color: #52c41a; }
.status-cancelled { background: #fff1f0; color: #f5222d; }
.empty {
  color: #999;
  font-size: 13px;
}
</style>
