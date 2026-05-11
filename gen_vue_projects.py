#!/usr/bin/env python3
"""Generate Vue and Full-stack project files for batch2"""
import os

BASE = r"d:\work\github\solo-coder\projects\trae-solo"
GI = "/target\n/dist\n/node_modules\n"

def w(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

VUE_PKG = '{"name":"{name}","version":"0.1.0","private":true,"scripts":{"dev":"vite","build":"vue-tsc -b && vite build","preview":"vite preview"},"dependencies":{"vue":"^3.5.0"},"devDependencies":{"@vitejs/plugin-vue":"^5.0.0","typescript":"^5.6.0","vite":"^6.0.0","vue-tsc":"^2.0.0"}}'

VITE_CFG = "import { defineConfig } from 'vite'\nimport vue from '@vitejs/plugin-vue'\n\nexport default defineConfig({\n  plugins: [vue()],\n})\n"

TS_CFG = '{"compilerOptions":{"target":"ES2020","module":"ESNext","moduleResolution":"bundler","strict":true,"jsx":"preserve","resolveJsonModule":true,"isolatedModules":true,"esModuleInterop":true,"lib":["ES2020","DOM"],"skipLibCheck":true,"noEmit":true},"include":["src/**/*.ts","src/**/*.tsx","src/**/*.vue"]}'

TS_NODE = '{"compilerOptions":{"target":"ES2022","module":"ESNext","moduleResolution":"bundler","allowSyntheticDefaultImports":true,"strict":true},"include":["vite.config.ts"]}'

HTML_TPL = '<!DOCTYPE html>\n<html lang="zh-CN"><head><meta charset="UTF-8"/><meta name="viewport" content="width=device-width,initial-scale=1.0"/><title>{title}</title></head>\n<body><div id="app"></div><script type="module" src="/src/main.ts"></script></body></html>'

MAIN_TS = "import { createApp } from 'vue'\nimport App from './App.vue'\ncreateApp(App).mount('#app')\n"

ENV_DTS = "/// <reference types=\"vite/client\" />\n"

vue_projects = {
    "approval-form-validation": {
        "title": "审批表单",
        "App.vue": '''<template>
  <div class="container">
    <h2>合同审批表单</h2>
    <form @submit.prevent="handleSubmit">
      <div class="field"><label>合同名称</label><input v-model="form.contractName" placeholder="请输入合同名称" /></div>
      <div class="field"><label>金额</label><input v-model.number="form.amount" type="number" placeholder="请输入金额" /></div>
      <div class="field"><label>审批人</label><select v-model="form.approver"><option value="">请选择</option><option value="manager">部门经理</option><option value="gm">总经理</option></select></div>
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
</style>'''
    },
    "workflow-designer": {
        "title": "工作流设计器",
        "App.vue": '''<template>
  <div class="designer">
    <h2>工作流设计器</h2>
    <div class="canvas">
      <div v-for="node in nodes" :key="node.id" class="node" :style="{ left: node.x + 'px', top: node.y + 'px' }">
        {{ node.label }}
      </div>
    </div>
    <!-- TODO: toolbar with draggable node types -->
    <!-- TODO: connection line drawing -->
    <!-- TODO: node deletion with Delete key -->
  </div>
</template>
<script setup lang="ts">
import { reactive } from 'vue'
interface WFNode { id: string; type: string; label: string; x: number; y: number }
const nodes = reactive<WFNode[]>([
  { id: 'start', type: 'start', label: '开始', x: 100, y: 50 },
  { id: 'approval', type: 'approval', label: '审批', x: 100, y: 150 },
  { id: 'end', type: 'end', label: '结束', x: 100, y: 250 },
])
</script>
<style>
.designer { padding: 20px; }
.canvas { position: relative; width: 800px; height: 400px; border: 1px solid #ddd; background: #fafafa; }
.node { position: absolute; padding: 10px 20px; background: #1890ff; color: white; border-radius: 4px; cursor: pointer; min-width: 80px; text-align: center; }
</style>'''
    },
    "master-detail-sync": {
        "title": "主子表",
        "App.vue": '''<template>
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
</style>'''
    },
    "permission-guard": {
        "title": "权限守卫",
        "App.vue": '''<template>
  <div class="container">
    <h2>审批系统</h2>
    <!-- BUG: no permission control -->
    <nav>
      <a href="#" @click.prevent="page='list'">列表</a>
      <a href="#" @click.prevent="page='submit'">提交</a>
      <a href="#" @click.prevent="page='approve'">审批</a>
      <a href="#" @click.prevent="page='report'">报表</a>
      <a href="#" @click.prevent="page='config'">配置</a>
    </nav>
    <div class="role-select">当前角色: <select v-model="currentRole">
      <option value="employee">普通员工</option><option value="manager">部门经理</option>
      <option value="gm">总经理</option><option value="admin">系统管理员</option>
    </select></div>
    <div class="content"><p>当前页面: {{ page }}</p>
      <button v-if="page==='list'">删除</button>
      <button v-if="page==='list'">审批</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const currentRole = ref('employee')
const page = ref('list')
</script>
<style>
.container { padding: 20px; }
nav { margin: 16px 0; display: flex; gap: 12px; }
nav a { padding: 6px 12px; background: #1890ff; color: white; text-decoration: none; border-radius: 4px; }
.role-select { margin: 12px 0; }
.content { padding: 16px; border: 1px solid #ddd; border-radius: 4px; }
button { margin: 4px; padding: 6px 16px; }
</style>'''
    },
    "notification-badge": {
        "title": "消息通知",
        "App.vue": '''<template>
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
const unreadCount = computed(() => notifications.value.filter(n => !n.read).length)
function markRead(id: number) { const n = notifications.value.find(n => n.id === id); if (n) n.read = true }
function clearAll() { notifications.value = [] }
</script>
<style>
.container { padding: 20px; position: relative; }
.bell { font-size: 24px; cursor: pointer; display: inline-block; position: relative; }
.badge { position: absolute; top: -8px; right: -8px; background: red; color: white; border-radius: 50%; padding: 2px 6px; font-size: 12px; }
.notification-list { position: absolute; top: 60px; right: 20px; width: 300px; border: 1px solid #ddd; background: white; border-radius: 4px; }
.item { padding: 10px; border-bottom: 1px solid #eee; cursor: pointer; }
.item.unread { background: #e6f7ff; }
</style>'''
    },
    "offline-queue": {
        "title": "离线队列",
        "App.vue": '''<template>
  <div class="container">
    <h2>审批客户端</h2>
    <!-- TODO: OfflineStatusBar component -->
    <div class="status" :class="{ offline: !isOnline }">
      {{ isOnline ? '在线' : '离线' }}
    </div>
    <form @submit.prevent="submitApproval">
      <div class="field"><label>合同编号</label><input v-model="form.contractId" /></div>
      <div class="field"><label>审批意见</label><input v-model="form.comment" /></div>
      <button type="submit">提交审批</button>
    </form>
    <!-- TODO: PendingQueue component -->
    <div v-if="pendingQueue.length > 0" class="queue">
      <h3>待同步队列 ({{ pendingQueue.length }})</h3>
      <div v-for="(item, idx) in pendingQueue" :key="idx" class="queue-item">
        {{ item.contractId }} - {{ item.comment }}
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { reactive, ref } from 'vue'
const isOnline = ref(navigator.onLine)
const form = reactive({ contractId: '', comment: '' })
const pendingQueue = reactive<{ contractId: string; comment: string }[]>([])

function submitApproval() {
  // BUG: fails completely when offline, no queue mechanism
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
</style>'''
    },
}

for abbr, proj in vue_projects.items():
    d = f"{BASE}/{abbr}"
    w(f"{d}/.gitignore", GI)
    w(f"{d}/package.json", VUE_PKG.replace("{name}", abbr))
    w(f"{d}/vite.config.ts", VITE_CFG)
    w(f"{d}/tsconfig.json", TS_CFG)
    w(f"{d}/tsconfig.node.json", TS_NODE)
    w(f"{d}/index.html", HTML_TPL.replace("{title}", proj["title"]))
    w(f"{d}/src/env.d.ts", ENV_DTS)
    w(f"{d}/src/main.ts", MAIN_TS)
    w(f"{d}/src/App.vue", proj["App.vue"])
    print(f"  Generated Vue: {abbr}")

# Full-stack projects (3)
fs_projects = {
    "api-contract": {
        "title": "API合同修复",
        "backend_main": r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub contract_name: String,
    pub sheet_amount: f64,  // BUG: backend expects snake_case "sheet_amount"
    pub approver: String,
    pub date: String,       // BUG: backend expects YYYY-MM-DD, frontend sends ISO
    pub urgent: String,     // BUG: backend expects bool, frontend sends string "true"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: String,
    pub status: String,
    pub message: String,
}

/// BUG: field names don't match frontend expectations
pub fn handle_approval(req: &ApprovalRequest) -> Result<ApprovalResponse> {
    // BUG: urgent is String but should be bool
    let _is_urgent = req.urgent == "true";
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
''',
        "frontend_app": '''<template>
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
</style>'''
    },
    "workflow-escalation": {
        "title": "审批超时升级",
        "backend_main": r'''use anyhow::Result;
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

/// TODO: check for timed-out approvals and escalate
pub fn check_escalation(tasks: &mut Vec<ApprovalTask>, timeout_hours: u64) -> Vec<String> {
    // TODO: not implemented - should check submitted_at + timeout > now
    vec![]
}

/// Get escalation target for a given level
pub fn get_escalation_target(level: u32) -> Option<String> {
    match level {
        1 => Some("manager".into()),
        2 => Some("gm".into()),
        3 => Some("admin".into()),
        _ => None,
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
''',
        "frontend_app": '''<template>
  <div class="container">
    <h2>审批列表</h2>
    <div v-for="task in tasks" :key="task.id" class="card" :class="{ escalated: task.escalated }">
      <div class="header">{{ task.id }} - {{ task.approver }}</div>
      <div class="status">{{ task.status }}</div>
      <!-- TODO: show escalation badge and reason -->
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const tasks = ref([
  { id: 't1', approver: 'zhangsan', status: 'pending', escalated: false },
  { id: 't2', approver: 'lisi', status: 'approved', escalated: false },
])
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.card { padding: 16px; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 12px; }
.card.escalated { border-color: #faad14; background: #fffbe6; }
.header { font-weight: bold; }
.status { color: #666; margin-top: 4px; }
</style>'''
    },
    "export-retry": {
        "title": "断点续传导出",
        "backend_main": r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub contract_name: String, pub amount: f64,
    pub status: String, pub approver: String, pub create_time: String,
}

/// TODO: implement paginated export with page_token
pub fn export_page(records: &[ApprovalRecord], page: usize, page_size: usize) -> (Vec<ApprovalRecord>, Option<String>) {
    let start = page * page_size;
    if start >= records.len() { return (vec![], None); }
    let end = std::cmp::min(start + page_size, records.len());
    let page_records = records[start..end].to_vec();
    let next_token = if end < records.len() { Some(format!("page_{}", page + 1)) } else { None };
    (page_records, next_token)
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
''',
        "frontend_app": '''<template>
  <div class="container">
    <h2>数据导出</h2>
    <div class="progress" v-if="exporting">
      <div class="bar" :style="{ width: progress + '%' }"></div>
      <span>{{ exported }} / {{ total }}</span>
    </div>
    <button @click="startExport" :disabled="exporting">开始导出</button>
    <button @click="cancelExport" v-if="exporting">取消</button>
    <!-- TODO: resume button after network failure -->
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const exporting = ref(false)
const exported = ref(0)
const total = ref(0)
const progress = ref(0)
function startExport() {
  // TODO: implement paginated download with progress
  exporting.value = true
  total.value = 5000
}
function cancelExport() { exporting.value = false }
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.progress { margin: 16px 0; }
.bar { height: 20px; background: #1890ff; border-radius: 4px; transition: width 0.3s; }
button { margin: 4px; padding: 8px 20px; }
</style>'''
    },
}

for abbr, proj in fs_projects.items():
    d = f"{BASE}/{abbr}"
    # Backend
    w(f"{d}/.gitignore", GI)
    w(f"{d}/backend/Cargo.toml", f'[package]\nname = "{abbr}-backend"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\nanyhow = "1.0"\nclap = {{ version = "4.5", features = ["derive"] }}\nserde = {{ version = "1.0", features = ["derive"] }}\nserde_json = "1.0"\n')
    w(f"{d}/backend/src/main.rs", proj["backend_main"])
    # Frontend
    w(f"{d}/frontend/package.json", VUE_PKG.replace("{name}", f"{abbr}-frontend"))
    w(f"{d}/frontend/vite.config.ts", VITE_CFG)
    w(f"{d}/frontend/tsconfig.json", TS_CFG)
    w(f"{d}/frontend/tsconfig.node.json", TS_NODE)
    w(f"{d}/frontend/index.html", HTML_TPL.replace("{title}", proj["title"]))
    w(f"{d}/frontend/src/env.d.ts", ENV_DTS)
    w(f"{d}/frontend/src/main.ts", MAIN_TS)
    w(f"{d}/frontend/src/App.vue", proj["frontend_app"])
    print(f"  Generated FS: {abbr}")

print("All Vue + Full-stack projects generated")
