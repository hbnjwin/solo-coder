<template>
  <div class="app">
    <header class="app-header">
      <h1>工作流设计器</h1>
      <div class="header-actions">
        <button class="btn" @click="onConnect">连接选中节点</button>
      </div>
    </header>
    <div class="app-body">
      <DesignerToolbar />
      <DesignerCanvas />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useDesigner } from './composables/useDesigner'
import DesignerToolbar from './components/DesignerToolbar.vue'
import DesignerCanvas from './components/DesignerCanvas.vue'

const { nodes, selectedNodeId, addConnection } = useDesigner()

let lastSelectedId: string | null = null

function onConnect(): void {
  if (lastSelectedId && selectedNodeId.value && lastSelectedId !== selectedNodeId.value) {
    addConnection(lastSelectedId, selectedNodeId.value)
    lastSelectedId = null
  } else {
    lastSelectedId = selectedNodeId.value
  }
}
</script>

<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
.app { display: flex; flex-direction: column; height: 100vh; }
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  background: #fff;
  border-bottom: 1px solid #e8e8e8;
}
.app-header h1 { font-size: 18px; font-weight: 600; color: #333; }
.header-actions { display: flex; gap: 8px; }
.btn {
  padding: 6px 14px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  background: #fff;
  font-size: 13px;
  cursor: pointer;
  transition: border-color 0.2s;
}
.btn:hover { border-color: #1890ff; color: #1890ff; }
.app-body { display: flex; flex: 1; overflow: hidden; }
</style>
