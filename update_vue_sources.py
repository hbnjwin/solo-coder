#!/usr/bin/env python3
"""Update Vue source files with hidden bugs matching new prompts"""
import os

BASE = r"d:\work\github\solo-coder\projects\trae-solo"

def w(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

# q14: approval-form-validation - add missing GM option bug
w(f"{BASE}/approval-form-validation/src/App.vue", '''<template>
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
</style>''')

# q15: workflow-designer - add cycle detection note
# (no code change needed - the constraint is in the prompt, current code is just static display)

# q16: master-detail-sync - add race condition note in code
w(f"{BASE}/master-detail-sync/src/App.vue", '''<template>
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
        <!-- BUG: detail table does not always refresh when selectedId changes -->
        <!-- BUG: fast consecutive clicks cause flickering / wrong data briefly -->
        <table><thead><tr><th>期次</th><th>金额</th><th>状态</th></tr></thead>
        <tbody><tr v-for="d in details" :key="d.period"><td>{{ d.period }}</td><td>{{ d.amount }}</td><td>{{ d.status }}</td></tr></tbody></table>
        <!-- TODO: loading state -->
        <!-- TODO: empty state -->
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
// BUG: computed doesn't handle race condition on fast clicks
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
</style>''')

# q17: permission-guard - add self-delete note
# (constraint is in prompt, code already shows no permission control)

# q18: notification-badge - add badge reset bug
w(f"{BASE}/notification-badge/src/App.vue", '''<template>
  <div class="container">
    <h2>消息通知</h2>
    <div class="bell" @click="showList = !showList">
      🔔 <span v-if="unreadCount > 0" class="badge">{{ unreadCount }}</span>
    </div>
    <div v-if="showList" class="notification-list">
      <div v-for="n in notifications" :key="n.id" class="item" :class="{ unread: !n.read }" @click="markRead(n.id)">
        <div class="title">{{ n.title }}</div>
        <div class="meta">{{ n.type }} · {{ n.time }}</div>
      </div>
      <button @click="clearAll">清空</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
interface Notif { id: number; title: string; type: string; time: string; read: boolean }
const showList = ref(false)
const notifications = ref<Notif[]>([
  { id: 1, title: '合同A等待审批', type: '审批', time: '10:00', read: false },
  { id: 2, title: '系统维护通知', type: '系统', time: '09:30', read: false },
  { id: 3, title: '合同B已通过', type: '审批', time: '昨天', read: true },
])
// BUG: badge count not reactive after clearAll
const unreadCount = computed(() => notifications.value.filter(n => !n.read).length)
function markRead(id: number) { const n = notifications.value.find(n => n.id === id); if (n) n.read = true }
// BUG: clearAll empties the list but badge may still show stale count
function clearAll() { notifications.value = [] }
// TODO: setInterval for real-time push simulation
// TODO: handle re-push of already-read notifications
</script>
<style>
.container { padding: 20px; position: relative; }
.bell { font-size: 24px; cursor: pointer; display: inline-block; position: relative; }
.badge { position: absolute; top: -8px; right: -8px; background: red; color: white; border-radius: 50%; padding: 2px 6px; font-size: 12px; }
.notification-list { position: absolute; top: 60px; right: 20px; width: 300px; border: 1px solid #ddd; background: white; border-radius: 4px; }
.item { padding: 10px; border-bottom: 1px solid #eee; cursor: pointer; }
.item.unread { background: #e6f7ff; }
</style>''')

# q19: offline-queue - add queue limit and order bug
w(f"{BASE}/offline-queue/src/App.vue", '''<template>
  <div class="container">
    <h2>审批客户端</h2>
    <div class="status" :class="{ offline: !isOnline }">
      {{ isOnline ? '在线' : '离线' }}
    </div>
    <form @submit.prevent="submitApproval">
      <div class="field"><label>合同编号</label><input v-model="form.contractId" /></div>
      <div class="field"><label>审批意见</label><input v-model="form.comment" /></div>
      <button type="submit">提交审批</button>
    </form>
    <div v-if="pendingQueue.length > 0" class="queue">
      <h3>待同步队列 ({{ pendingQueue.length }})</h3>
      <div v-for="(item, idx) in pendingQueue" :key="idx" class="queue-item">
        {{ item.contractId }} - {{ item.comment }} <span class="time">{{ item.timestamp }}</span>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { reactive, ref } from 'vue'
const isOnline = ref(navigator.onLine)
const form = reactive({ contractId: '', comment: '' })
const pendingQueue = reactive<{ contractId: string; comment: string; timestamp: number }[]>([])

// BUG: fails completely when offline, no queue mechanism
// BUG: no queue limit - can grow unbounded
// BUG: sync order not guaranteed (would need timestamp-based sorting)
function submitApproval() {
  if (!isOnline.value) {
    alert('网络不可用，提交失败') // BUG: should queue instead
    return
  }
  console.log('submitted:', form)
}
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.status { padding: 8px 16px; background: #52c41a; color: white; border-radius: 4px; display: inline-block; margin-bottom: 16px; }
.status.offline { background: #ff4d4f; }
.field { margin-bottom: 16px; }
.field label { display: block; margin-bottom: 4px; font-weight: bold; }
.field input { width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; }
.queue { margin-top: 20px; padding: 16px; border: 1px solid #faad14; border-radius: 4px; background: #fffbe6; }
.queue-item { padding: 8px; border-bottom: 1px solid #ffe58f; }
.time { color: #999; font-size: 12px; margin-left: 8px; }
</style>''')

# Full-stack projects - update backend main.rs with hidden bugs

# q20: api-contract - urgent field is String not bool
w(f"{BASE}/api-contract/backend/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub contract_name: String,
    pub sheet_amount: f64,  // BUG: backend expects snake_case "sheet_amount"
    pub approver: String,
    pub date: String,       // BUG: backend expects YYYY-MM-DD, frontend sends ISO
    pub urgent: String,     // BUG: should be bool but is String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: String,
    pub status: String,
    pub message: String,
}

/// BUG: urgent is String but should be bool, uses == "true" hack
pub fn handle_approval(req: &ApprovalRequest) -> Result<ApprovalResponse> {
    let _is_urgent = req.urgent == "true"; // BUG: fragile string comparison
    // BUG: date parsing assumes YYYY-MM-DD but frontend sends ISO format
    Ok(ApprovalResponse { id: "1".into(), status: "created".into(), message: "ok".into() })
}

#[derive(Parser, Debug)]
#[command(name = "api-contract-backend", about = "Approval API backend")]
struct Cli {}

fn main() -> Result<()> {
    let req = ApprovalRequest {
        contract_name: "Contract A".into(), sheet_amount: 500000.0,
        approver: "zhangsan".into(), date: "2026-05-11T10:00:00Z".into(), urgent: "true".into(),
    };
    let resp = handle_approval(&req)?;
    println!("{}", serde_json::to_string_pretty(&resp)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgent_should_be_bool() {
        let req = ApprovalRequest {
            contract_name: "test".into(), sheet_amount: 100.0,
            approver: "a".into(), date: "2026-01-01".into(), urgent: "true".into(),
        };
        // BUG: urgent field is String, should be bool
        // After fix: req.urgent should be true (bool), not "true" (String)
        assert!(req.urgent == "true", "urgent should be bool type, not string comparison");
    }

    #[test]
    fn test_date_format_mismatch() {
        // Frontend sends ISO: 2026-05-11T10:00:00Z
        // Backend expects: 2026-05-11
        let req = ApprovalRequest {
            contract_name: "test".into(), sheet_amount: 100.0,
            approver: "a".into(), date: "2026-05-11T10:00:00Z".into(), urgent: "true".into(),
        };
        // BUG: date contains T which means ISO format, backend should normalize
        assert!(!req.date.contains('T'), "date should be normalized to YYYY-MM-DD format");
    }
}
''')

# q21: workflow-escalation - add no-upper-limit bug
w(f"{BASE}/workflow-escalation/backend/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTask {
    pub id: String,
    pub approver: String,
    pub approver_level: u32, // 1=employee, 2=manager, 3=gm, 4=admin
    pub status: String,
    pub submitted_at: String,
    pub escalated: bool,
}

/// BUG: not implemented - should check submitted_at + timeout > now
pub fn check_escalation(tasks: &mut Vec<ApprovalTask>, timeout_hours: u64) -> Vec<String> {
    // TODO: implement
    vec![]
}

/// BUG: no upper limit check - if admin doesn't handle, escalation continues forever
pub fn get_escalation_target(level: u32) -> Option<String> {
    match level {
        1 => Some("manager".into()),
        2 => Some("gm".into()),
        3 => Some("admin".into()),
        // BUG: level 4+ should return None (no more escalation) but doesn't
        _ => Some("super_admin".into()), // BUG: creates infinite escalation chain
    }
}

#[derive(Parser, Debug)]
#[command(name = "workflow-escalation", about = "Approval timeout escalation")]
struct Cli { #[arg(long, default_value = "24")] timeout_hours: u64 }

fn main() -> Result<()> {
    let mut tasks = vec![ApprovalTask { id: "t1".into(), approver: "zhangsan".into(), approver_level: 2, status: "pending".into(), submitted_at: "2026-05-10T08:00:00Z".into(), escalated: false }];
    let escalated = check_escalation(&mut tasks, 24);
    println!("escalated: {:?}", escalated);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation_has_upper_limit() {
        // Level 4 (admin) should not escalate further
        let target = get_escalation_target(4);
        assert!(target.is_none(), "admin level should have no further escalation target");
    }

    #[test]
    fn test_check_escalation_not_implemented() {
        let mut tasks = vec![ApprovalTask { id: "t1".into(), approver: "zhangsan".into(), approver_level: 2, status: "pending".into(), submitted_at: "2026-01-01T00:00:00Z".into(), escalated: false }];
        let escalated = check_escalation(&mut tasks, 24);
        // Should detect timeout but returns empty
        assert!(!escalated.is_empty(), "should detect timed-out task");
    }
}
''')

# q22: export-retry - add data drift bug (page-based instead of cursor-based)
w(f"{BASE}/export-retry/backend/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub contract_name: String, pub amount: f64,
    pub status: String, pub approver: String, pub create_time: String,
}

/// BUG: uses page number instead of cursor, causing data drift when new records are inserted during export
pub fn export_page(records: &[ApprovalRecord], page: usize, page_size: usize) -> (Vec<ApprovalRecord>, Option<String>) {
    let start = page * page_size;
    if start >= records.len() { return (vec![], None); }
    let end = std::cmp::min(start + page_size, records.len());
    let page_records = records[start..end].to_vec();
    // BUG: page_token is just "page_N" which breaks if data changes between pages
    let next_token = if end < records.len() { Some(format!("page_{}", page + 1)) } else { None };
    (page_records, next_token)
}

/// TODO: should use cursor-based pagination (last record ID as token)
pub fn export_cursor(records: &[ApprovalRecord], cursor: Option<&str>, page_size: usize) -> (Vec<ApprovalRecord>, Option<String>) {
    // TODO: implement cursor-based pagination
    (vec![], None)
}

#[derive(Parser, Debug)]
#[command(name = "export-retry", about = "Export with pagination and retry")]
struct Cli { #[arg(long, default_value = "1000")] page_size: usize }

fn main() -> Result<()> {
    let records: Vec<ApprovalRecord> = (0..5000).map(|i| ApprovalRecord {
        id: format!("{}", i), contract_name: format!("Contract {}", i), amount: i as f64 * 1000.0,
        status: if i % 3 == 0 { "approved".into() } else { "pending".into() },
        approver: "zhangsan".into(), create_time: "2026-01-01".into(),
    }).collect();
    let (page, token) = export_page(&records, 0, 1000);
    println!("page 0: {} records, next_token: {:?}", page.len(), token);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_based_drift() {
        // Simulate: page 0 returns records 0-9, then a new record is inserted at position 0
        // Page 1 would return records 10-19 shifted by 1, causing record 10 to appear twice
        // This test documents the bug
        let records: Vec<ApprovalRecord> = (0..20).map(|i| ApprovalRecord {
            id: format!("{}", i), contract_name: format!("C{}", i), amount: 100.0,
            status: "ok".into(), approver: "z".into(), create_time: "2026-01-01".into(),
        }).collect();
        let (page0, _) = export_page(&records, 0, 10);
        let (page1, _) = export_page(&records, 1, 10);
        // Check for overlap
        let ids0: Vec<_> = page0.iter().map(|r| r.id.clone()).collect();
        let ids1: Vec<_> = page1.iter().map(|r| r.id.clone()).collect();
        let overlap: Vec<_> = ids0.iter().filter(|id| ids1.contains(id)).collect();
        assert!(overlap.is_empty(), "page-based pagination should not have overlap but does: {:?}", overlap);
    }

    #[test]
    fn test_cursor_based_not_implemented() {
        let records: Vec<ApprovalRecord> = (0..20).map(|i| ApprovalRecord {
            id: format!("{}", i), contract_name: format!("C{}", i), amount: 100.0,
            status: "ok".into(), approver: "z".into(), create_time: "2026-01-01".into(),
        }).collect();
        let (page, token) = export_cursor(&records, None, 10);
        assert!(!page.is_empty(), "cursor-based export should return records");
        assert!(token.is_some(), "cursor-based export should return next cursor");
    }
}
''')

print("All Vue + Full-stack source files updated with hidden bugs")
