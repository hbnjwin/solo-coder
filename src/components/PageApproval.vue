<template>
  <div class="page-approval">
    <h2>审批管理</h2>
    <table class="approval-table">
      <thead>
        <tr>
          <th>申请编号</th>
          <th>申请人</th>
          <th>类型</th>
          <th>状态</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in approvalList" :key="item.id">
          <td>{{ item.id }}</td>
          <td>{{ item.applicant }}</td>
          <td>{{ item.type }}</td>
          <td>{{ item.status }}</td>
          <td>
            <PermissionGuard permission="approve">
              <button class="btn btn-approve" @click="handleApprove(item.id)">通过</button>
            </PermissionGuard>
            <PermissionGuard permission="reject">
              <button class="btn btn-reject" @click="handleReject(item.id)">驳回</button>
            </PermissionGuard>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PermissionGuard from './PermissionGuard.vue'

interface ApprovalItem {
  id: string
  applicant: string
  type: string
  status: string
}

const approvalList = ref<ApprovalItem[]>([
  { id: 'AP-001', applicant: '李四', type: '请假', status: '待审批' },
  { id: 'AP-002', applicant: '王五', type: '报销', status: '待审批' },
  { id: 'AP-003', applicant: '赵六', type: '采购', status: '已通过' },
])

function handleApprove(id: string) {
  const item = approvalList.value.find((i) => i.id === id)
  if (item) item.status = '已通过'
}

function handleReject(id: string) {
  const item = approvalList.value.find((i) => i.id === id)
  if (item) item.status = '已驳回'
}
</script>

<style scoped>
.page-approval { padding: 20px; }
.approval-table { width: 100%; border-collapse: collapse; margin-top: 16px; }
.approval-table th, .approval-table td { padding: 10px 12px; border: 1px solid #e8e8e8; text-align: left; }
.approval-table th { background: #fafafa; font-weight: 600; }
.btn { padding: 4px 12px; border: none; border-radius: 4px; cursor: pointer; margin-right: 6px; }
.btn-approve { background: #52c41a; color: #fff; }
.btn-reject { background: #ff4d4f; color: #fff; }
</style>
