<template>
  <div class="page-settings">
    <h2>系统设置</h2>
    <PermissionGuard permission="configure">
      <div class="settings-section">
        <h3>审批流程配置</h3>
        <div class="setting-item">
          <label>自动审批阈值 (元)</label>
          <input type="number" v-model="threshold" />
        </div>
        <div class="setting-item">
          <label>审批超时 (小时)</label>
          <input type="number" v-model="timeout" />
        </div>
        <button class="btn-save" @click="saveSettings">保存设置</button>
      </div>
    </PermissionGuard>
    <PermissionGuard permission="manage_users">
      <div class="settings-section">
        <h3>用户管理</h3>
        <p>当前系统用户数: 42</p>
        <button class="btn-save" @click="manageUsers">管理用户</button>
      </div>
    </PermissionGuard>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PermissionGuard from './PermissionGuard.vue'

const threshold = ref(5000)
const timeout = ref(48)

function saveSettings() {
  alert(`设置已保存: 阈值=${threshold.value}, 超时=${timeout.value}h`)
}

function manageUsers() {
  alert('打开用户管理面板')
}
</script>

<style scoped>
.page-settings { padding: 20px; }
.settings-section { margin-top: 20px; padding: 16px; background: #fafafa; border-radius: 8px; border: 1px solid #e8e8e8; }
.settings-section h3 { margin: 0 0 12px; }
.setting-item { display: flex; align-items: center; gap: 12px; margin-bottom: 12px; }
.setting-item label { min-width: 160px; }
.setting-item input { padding: 6px 10px; border: 1px solid #d9d9d9; border-radius: 4px; width: 120px; }
.btn-save { padding: 6px 16px; background: #1890ff; color: #fff; border: none; border-radius: 4px; cursor: pointer; margin-top: 8px; }
</style>
