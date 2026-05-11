<template>
  <div class="contract-detail">
    <h3>付款明细</h3>
    <LoadingSpinner :visible="loading" text="加载明细..." />
    <div v-if="error" class="error-msg">{{ error }}</div>
    <table v-if="!loading && !error && details.length">
      <thead>
        <tr>
          <th>期次</th>
          <th>金额</th>
          <th>到期日</th>
          <th>状态</th>
          <th>付款日</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in details" :key="item.period">
          <td>第{{ item.period }}期</td>
          <td>¥{{ item.amount.toLocaleString() }}</td>
          <td>{{ item.dueDate }}</td>
          <td>
            <span :class="['payment-status', paymentClass(item.status)]">
              {{ item.status }}
            </span>
          </td>
          <td>{{ item.paidDate || '-' }}</td>
        </tr>
      </tbody>
      <tfoot>
        <tr>
          <td>合计</td>
          <td>¥{{ totalAmount.toLocaleString() }}</td>
          <td colspan="3"></td>
        </tr>
      </tfoot>
    </table>
    <p v-if="!loading && !error && !details.length && selectedId" class="empty">
      暂无付款明细
    </p>
    <p v-if="!selectedId" class="hint">请从左侧选择一个合同查看明细</p>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'
import type { ContractDetail } from '../types'
import { useContractDetail } from '../composables/useContractDetail'
import LoadingSpinner from './LoadingSpinner.vue'

const props = defineProps<{
  selectedId: string
}>()

const { details, loading, error, loadDetail } = useContractDetail()

watch(
  () => props.selectedId,
  (newId) => {
    loadDetail(newId)
  },
  { immediate: false }
)

const totalAmount = computed(() => {
  return details.value.reduce((sum: number, d: ContractDetail) => sum + d.amount, 0)
})

function paymentClass(status: string): string {
  switch (status) {
    case '已付': return 'paid'
    case '未付': return 'unpaid'
    case '逾期': return 'overdue'
    default: return ''
  }
}
</script>

<style scoped>
.contract-detail {
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
tfoot td {
  font-weight: 600;
  background: #fafafa;
}
.payment-status {
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 12px;
}
.paid { background: #f6ffed; color: #52c41a; }
.unpaid { background: #fffbe6; color: #faad14; }
.overdue { background: #fff1f0; color: #f5222d; }
.error-msg {
  color: #f5222d;
  font-size: 13px;
  padding: 8px 0;
}
.empty, .hint {
  color: #999;
  font-size: 13px;
}
</style>
