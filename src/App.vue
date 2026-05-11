<template>
  <div class="app-container">
    <header class="app-header">
      <h1>审批客户端</h1>
      <NetworkIndicator />
    </header>
    <main class="app-main">
      <ApprovalForm />
      <SyncProgress />
      <QueuePanel />
    </main>
  </div>
</template>

<script setup lang="ts">
import NetworkIndicator from './components/NetworkIndicator.vue'
import ApprovalForm from './components/ApprovalForm.vue'
import QueuePanel from './components/QueuePanel.vue'
import SyncProgress from './components/SyncProgress.vue'
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
  color: #333;
}
.app-container {
  max-width: 560px;
  margin: 40px auto;
  padding: 24px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}
.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid #f0f0f0;
}
.app-header h1 {
  font-size: 20px;
  font-weight: 600;
}
</style>
