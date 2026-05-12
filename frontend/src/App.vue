<template>
  <div class="monitor-page">
    <header class="page-header">
      <h1>审批超时监控</h1>
    </header>
    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-label">待审批数量</div>
        <div class="stat-value">{{ pendingList.length }}</div>
      </div>
      <div class="stat-card warning">
        <div class="stat-label">超时预警数量</div>
        <div class="stat-value">{{ escalationList.filter(e => !e.acknowledged).length }}</div>
      </div>
    </div>
    <div class="section">
      <h2>待审批列表</h2>
      <table class="data-table">
        <thead>
          <tr>
            <th>请求ID</th>
            <th>申请人</th>
            <th>当前审批人</th>
            <th>已等待</th>
            <th>状态</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in pendingList" :key="item.id" :class="{ 'timeout-row': isTimeout(item) }">
            <td>{{ item.id }}</td>
            <td>{{ item.applicant }}</td>
            <td>{{ item.current_approver }}</td>
            <td>{{ formatElapsed(item.created_at) }}</td>
            <td><span class="status-badge" :class="item.status.toLowerCase()">{{ item.status }}</span></td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="section">
      <h2>升级事件</h2>
      <table class="data-table">
        <thead>
          <tr>
            <th>事件ID</th>
            <th>请求ID</th>
            <th>从</th>
            <th>到</th>
            <th>升级级别</th>
            <th>原因</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="evt in escalationList" :key="evt.id" :class="{ 'acked': evt.acknowledged }">
            <td>{{ evt.id }}</td>
            <td>{{ evt.request_id }}</td>
            <td>{{ evt.from_approver }}</td>
            <td>{{ evt.to_approver }}</td>
            <td>L{{ evt.escalation_level }}</td>
            <td>{{ evt.reason }}</td>
            <td>
              <button v-if="!evt.acknowledged" @click="ackEvent(evt.id)" class="ack-btn">确认</button>
              <span v-else class="acked-text">已确认</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface ApprovalRequest {
  id: string
  applicant: string
  current_approver: string
  original_approver: string
  escalation_level: number
  created_at: string
  status: string
}

interface EscalationEvent {
  id: string
  request_id: string
  from_approver: string
  to_approver: string
  escalation_level: number
  reason: string
  created_at: string
  acknowledged: boolean
}

const pendingList = ref<ApprovalRequest[]>([])
const escalationList = ref<EscalationEvent[]>([])
let refreshTimer: ReturnType<typeof setInterval> | null = null

async function fetchPending() {
  try {
    const res = await fetch('/api/pending')
    pendingList.value = await res.json()
  } catch {
    pendingList.value = []
  }
}

async function fetchEscalations() {
  try {
    const res = await fetch('/api/escalations')
    escalationList.value = await res.json()
  } catch {
    escalationList.value = []
  }
}

async function ackEvent(id: string) {
  await fetch(`/api/escalations/${id}/ack`, { method: 'POST' })
  await fetchEscalations()
}

function isTimeout(item: ApprovalRequest): boolean {
  const elapsed = Date.now() - new Date(item.created_at).getTime()
  return elapsed > 60 * 60 * 1000
}

function formatElapsed(createdAt: string): string {
  const elapsed = Date.now() - new Date(createdAt).getTime()
  const minutes = Math.floor(elapsed / 60000)
  if (minutes < 60) return `${minutes}分钟`
  const hours = Math.floor(minutes / 60)
  return `${hours}小时${minutes % 60}分钟`
}

async function refreshAll() {
  await Promise.all([fetchPending(), fetchEscalations()])
}

onMounted(() => {
  refreshAll()
  refreshTimer = setInterval(refreshAll, 10000)
})

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>

<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f5f5f5; }
.monitor-page { max-width: 1200px; margin: 0 auto; padding: 24px; }
.page-header { margin-bottom: 24px; }
.page-header h1 { font-size: 20px; color: #333; }
.stats-row { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 24px; }
.stat-card { background: #fff; border-radius: 8px; padding: 20px; border-left: 4px solid #1890ff; }
.stat-card.warning { border-left-color: #faad14; }
.stat-label { font-size: 13px; color: #999; margin-bottom: 8px; }
.stat-value { font-size: 28px; font-weight: 700; color: #333; }
.section { background: #fff; border-radius: 8px; padding: 20px; margin-bottom: 16px; }
.section h2 { font-size: 16px; color: #333; margin-bottom: 16px; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th, .data-table td { padding: 10px 12px; text-align: left; font-size: 13px; border-bottom: 1px solid #f0f0f0; }
.data-table th { color: #999; font-weight: 500; }
.timeout-row { background: #fff2f0; }
.status-badge { display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 12px; }
.status-badge.pending { background: #e6f7ff; color: #1890ff; }
.status-badge.escalated { background: #fff2e8; color: #fa8c16; }
.ack-btn { padding: 4px 12px; background: #1890ff; color: #fff; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; }
.ack-btn:hover { background: #40a9ff; }
.acked { opacity: 0.6; }
.acked-text { color: #52c41a; font-size: 12px; }
</style>
