<template>
  <div class="node-panel">
    <h3>节点面板</h3>
    <div
      v-for="type in nodeTypes"
      :key="type.id"
      class="node-item"
      draggable="true"
      @dragstart="onDragStart($event, type)"
    >
      <span class="node-icon">{{ type.icon }}</span>
      <span>{{ type.label }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
const nodeTypes = [
  { id: 'approval', label: '审批节点', icon: '✅' },
  { id: 'condition', label: '条件分支', icon: '🔀' },
  { id: 'countersign', label: '会签节点', icon: '👥' },
  { id: 'cc', label: '抄送节点', icon: '📨' },
]

function onDragStart(event: DragEvent, type: { id: string; label: string; icon: string }) {
  event.dataTransfer?.setData('nodeType', JSON.stringify(type))
}
</script>

<style scoped>
.node-panel {
  width: 200px;
  background: #fff;
  border-right: 1px solid #e0e0e0;
  padding: 16px;
}
.node-panel h3 {
  font-size: 14px;
  color: #333;
  margin-bottom: 12px;
}
.node-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  margin-bottom: 8px;
  background: #f7f8fa;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  cursor: grab;
  font-size: 13px;
}
.node-item:hover {
  border-color: #1890ff;
  background: #e6f7ff;
}
.node-icon {
  font-size: 16px;
}
</style>
