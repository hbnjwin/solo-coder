<template>
  <div
    ref="canvasRef"
    class="designer-canvas"
    @dragover="onDragOver"
    @drop="onDrop"
    @click="onCanvasClick"
  >
    <!-- FIXME: SVG viewBox may need recalculation on resize -->
    <ConnectionLine
      v-for="conn in connections"
      :key="conn.id"
      :source-pos="getNodeById(conn.sourceId)?.position"
      :target-pos="getNodeById(conn.targetId)?.position"
    />
    <WorkflowNodeComp
      v-for="node in nodes"
      :key="node.id"
      :node="node"
      :is-selected="selectedNodeId === node.id"
      @select="selectNode"
      @delete="deleteNode"
      @drag-start="onNodeDragStart"
    />
    <div v-if="nodes.length === 0" class="canvas-empty">
      将节点从左侧拖入画布
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useDesigner } from '../composables/useDesigner'
import { useDragDrop } from '../composables/useDragDrop'
import ConnectionLine from './ConnectionLine.vue'
import WorkflowNodeComp from './WorkflowNode.vue'

const canvasRef = ref<HTMLElement | null>(null)

const { nodes, connections, selectedNodeId, selectNode, deleteNode, updateNodePosition, getNodeById } = useDesigner()
const { onDragOver, onDrop } = useDragDrop()

let draggingNodeId: string | null = null
let dragOffset = { x: 0, y: 0 }

function onCanvasClick(): void {
  selectNode(null)
}

function onNodeDragStart(nodeId: string, event: MouseEvent): void {
  const node = getNodeById(nodeId)
  if (!node) return

  draggingNodeId = nodeId
  dragOffset.x = event.clientX - node.position.x
  dragOffset.y = event.clientY - node.position.y

  document.addEventListener('mousemove', onNodeDrag)
  document.addEventListener('mouseup', onNodeDragEnd)
}

function onNodeDrag(event: MouseEvent): void {
  if (!draggingNodeId) return
  const x = event.clientX - dragOffset.x
  const y = event.clientY - dragOffset.y
  updateNodePosition(draggingNodeId, { x, y })
}

function onNodeDragEnd(): void {
  draggingNodeId = null
  document.removeEventListener('mousemove', onNodeDrag)
  document.removeEventListener('mouseup', onNodeDragEnd)
}

function onKeyDown(event: KeyboardEvent): void {
  if (event.key === 'Delete' && selectedNodeId.value) {
    deleteNode(selectedNodeId.value)
  }
}

onMounted(() => {
  document.addEventListener('keydown', onKeyDown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', onKeyDown)
})
</script>

<style scoped>
.designer-canvas {
  flex: 1;
  position: relative;
  background: #fafafa;
  background-image:
    linear-gradient(rgba(0, 0, 0, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(0, 0, 0, 0.03) 1px, transparent 1px);
  background-size: 20px 20px;
  overflow: hidden;
  min-height: 500px;
}
.canvas-empty {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #bbb;
  font-size: 14px;
  pointer-events: none;
}
</style>
