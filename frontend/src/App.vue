<template>
  <div class="app-container">
    <h1>审批超时升级管理</h1>
    <EscalationList :items="escalationItems" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { EscalationItem } from './types'
import EscalationList from './components/EscalationList.vue'

const escalationItems = ref<EscalationItem[]>([
  {
    id: 'req-001',
    title: '采购申请 - 办公设备',
    approver: 'zhangsan',
    submittedAt: '2026-05-10T08:00:00Z',
    status: 'pending',
    escalationLevel: 0,
    resolvedAt: null,
  },
  {
    id: 'req-002',
    title: '出差报销 - 上海客户拜访',
    approver: 'lisi',
    submittedAt: '2026-05-09T14:30:00Z',
    status: 'escalated',
    escalationLevel: 2,
    resolvedAt: null,
  },
  {
    id: 'req-003',
    title: '人员招聘 - 后端工程师',
    approver: 'wangwu',
    submittedAt: '2026-05-08T09:00:00Z',
    status: 'approved',
    escalationLevel: 1,
    resolvedAt: '2026-05-09T10:00:00Z',
  },
])
</script>

<style>
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #fafafa;
}
.app-container {
  max-width: 720px;
  margin: 0 auto;
  padding: 32px 20px;
}
h1 {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 24px;
  color: #262626;
}
</style>
