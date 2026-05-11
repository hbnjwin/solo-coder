<template>
  <div id="app-root">
    <header class="app-header">
      <nav class="tab-nav">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="['tab-btn', { active: currentTab === tab.id }]"
          @click="currentTab = tab.id"
        >
          {{ tab.label }}
        </button>
      </nav>
    </header>
    <main class="app-main">
      <ApprovalForm v-if="currentTab === 'form'" />
      <div v-else-if="currentTab === 'history'" class="placeholder">
        <p>审批历史记录（开发中）</p>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ApprovalForm from './components/ApprovalForm.vue'

const tabs = [
  { id: 'form', label: '新建审批' },
  { id: 'history', label: '审批历史' },
]

const currentTab = ref('form')
</script>

<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f5f5f5; color: #333; }
#app-root { min-height: 100vh; }
.app-header { background: white; border-bottom: 1px solid #e8e8e8; padding: 0 24px; }
.tab-nav { display: flex; gap: 0; }
.tab-btn { padding: 14px 20px; border: none; background: none; cursor: pointer; font-size: 14px; color: #666; border-bottom: 2px solid transparent; transition: all 0.2s; }
.tab-btn.active { color: #1890ff; border-bottom-color: #1890ff; }
.tab-btn:hover { color: #1890ff; }
.app-main { padding: 32px 24px; }
.placeholder { text-align: center; padding: 60px; color: #999; }
</style>
