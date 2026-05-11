<template>
  <div class="container">
    <h2>合同管理</h2>
    <div class="layout">
      <div class="master">
        <h3>合同列表</h3>
        <table><thead><tr><th>编号</th><th>名称</th><th>金额</th></tr></thead>
        <tbody><tr v-for="c in contracts" :key="c.id" :class="{ selected: selectedId === c.id }" @click="selectContract(c.id)">
          <td>{{ c.id }}</td><td>{{ c.name }}</td><td>{{ c.amount }}</td>
        </tr></tbody></table>
      </div>
      <div class="detail">
        <h3>付款明细</h3>
        <!-- BUG: detail table does not always refresh -->
        <table><thead><tr><th>期次</th><th>金额</th><th>状态</th></tr></thead>
        <tbody><tr v-for="d in details" :key="d.period"><td>{{ d.period }}</td><td>{{ d.amount }}</td><td>{{ d.status }}</td></tr></tbody></table>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
interface Contract { id: string; name: string; amount: number }
interface Detail { period: number; amount: number; status: string }
const contracts: Contract[] = [
  { id: 'C001', name: '合同A', amount: 500000 },
  { id: 'C002', name: '合同B', amount: 2000000 },
  { id: 'C003', name: '合同C', amount: 800000 },
]
const allDetails: Record<string, Detail[]> = {
  C001: [{ period: 1, amount: 250000, status: '已付' }, { period: 2, amount: 250000, status: '未付' }],
  C002: [{ period: 1, amount: 1000000, status: '已付' }],
  C003: [{ period: 1, amount: 800000, status: '未付' }],
}
const selectedId = ref('')
const details = computed(() => { if (!selectedId.value) return []; return allDetails[selectedId.value] || [] })
function selectContract(id: string) { selectedId.value = id }
</script>
<style>
.container { padding: 20px; }
.layout { display: flex; gap: 20px; }
.master, .detail { flex: 1; }
table { width: 100%; border-collapse: collapse; }
th, td { padding: 8px 12px; border: 1px solid #ddd; }
.selected { background: #e6f7ff; }
</style>