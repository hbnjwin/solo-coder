<template>
  <div class="toolbar">
    <h3 class="toolbar-title">节点类型</h3>
    <div
      v-for="nodeType in NODE_TYPES"
      :key="nodeType.type"
      class="toolbar-item"
      draggable="true"
      :style="{ borderLeftColor: nodeType.color }"
      @dragstart="handleDragStart($event, nodeType.type, nodeType.label)"
      @dragend="onDragEnd"
    >
      <span class="toolbar-item-dot" :style="{ background: nodeType.color }"></span>
      {{ nodeType.label }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { NODE_TYPES, type NodeType } from '../types'
import { useDragDrop } from '../composables/useDragDrop'

const { onDragStart, onDragEnd } = useDragDrop()

function handleDragStart(event: DragEvent, type: NodeType, label: string): void {
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'copy'
    event.dataTransfer.setData('text/plain', type)
  }
  onDragStart(type, label)
}
</script>

<style scoped>
.toolbar {
  width: 180px;
  padding: 16px 12px;
  background: #fff;
  border-right: 1px solid #e8e8e8;
  user-select: none;
}
.toolbar-title {
  margin: 0 0 12px;
  font-size: 14px;
  color: #333;
}
.toolbar-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  margin-bottom: 8px;
  border: 1px solid #e8e8e8;
  border-left: 3px solid;
  border-radius: 4px;
  cursor: grab;
  font-size: 13px;
  transition: background 0.2s;
}
.toolbar-item:hover {
  background: #f5f5f5;
}
.toolbar-item:active {
  cursor: grabbing;
}
.toolbar-item-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
</style>
