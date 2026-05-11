<template>
  <div
    class="workflow-node"
    :class="{ selected: isSelected }"
    :style="nodeStyle"
    @mousedown.stop="onMouseDown"
    @click.stop="onClick"
  >
    <span class="node-indicator" :style="{ background: nodeColor }"></span>
    <span class="node-label">{{ node.label }}</span>
    <button
      v-if="isSelected"
      class="node-delete"
      @click.stop="onDelete"
      title="删除节点"
    >&times;</button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { WorkflowNode } from '../types'
import { NODE_TYPES } from '../types'

const props = defineProps<{
  node: WorkflowNode
  isSelected: boolean
}>()

const emit = defineEmits<{
  select: [nodeId: string]
  delete: [nodeId: string]
  dragStart: [nodeId: string, event: MouseEvent]
}>()

const nodeColor = computed(() => {
  const config = NODE_TYPES.find(t => t.type === props.node.type)
  return config?.color ?? '#1890ff'
})

const nodeStyle = computed(() => ({
  transform: `translate(${props.node.position.x}px, ${props.node.position.y}px)`,
}))

function onMouseDown(event: MouseEvent): void {
  emit('dragStart', props.node.id, event)
}

function onClick(): void {
  emit('select', props.node.id)
}

function onDelete(): void {
  emit('delete', props.node.id)
}
</script>

<style scoped>
.workflow-node {
  position: absolute;
  top: 0;
  left: 0;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: #fff;
  border: 2px solid #d9d9d9;
  border-radius: 6px;
  cursor: move;
  font-size: 13px;
  white-space: nowrap;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  transition: border-color 0.15s, box-shadow 0.15s;
}
.workflow-node.selected {
  border-color: #1890ff;
  box-shadow: 0 0 0 3px rgba(24, 144, 255, 0.15);
}
.node-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}
.node-label {
  flex: 1;
}
.node-delete {
  margin-left: 6px;
  border: none;
  background: none;
  color: #ff4d4f;
  font-size: 16px;
  cursor: pointer;
  padding: 0 2px;
  line-height: 1;
}
.node-delete:hover {
  color: #cf1322;
}
</style>
