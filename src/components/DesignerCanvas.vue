<template>
  <div
    class="designer-canvas"
    @drop="onDrop"
    @dragover.prevent
    @wheel="onWheel"
    @mousedown="onCanvasMouseDown"
    @mousemove="onCanvasMouseMove"
    @mouseup="onCanvasMouseUp"
  >
    <svg class="connections-layer" :viewBox="viewBox">
      <line
        v-for="conn in connections"
        :key="conn.id"
        :x1="conn.x1"
        :y1="conn.y1"
        :x2="conn.x2"
        :y2="conn.y2"
        stroke="#999"
        stroke-width="2"
      />
    </svg>
    <div
      v-for="node in nodes"
      :key="node.id"
      class="canvas-node"
      :class="{ selected: selectedNodeId === node.id }"
      :style="{ left: node.x + 'px', top: node.y + 'px' }"
      @mousedown.stop="onNodeMouseDown($event, node.id)"
      @click.stop="selectNode(node.id)"
    >
      <div class="node-header">{{ node.label }}</div>
      <div class="node-type">{{ node.type }}</div>
      <div class="port port-out" @mousedown.stop="onPortMouseDown($event, node.id)" />
      <div class="port port-in" @mouseup.stop="onPortMouseUp($event, node.id)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface CanvasNode {
  id: string
  type: string
  label: string
  x: number
  y: number
}

interface Connection {
  id: string
  fromNodeId: string
  toNodeId: string
  x1: number
  y1: number
  x2: number
  y2: number
}

const nodes = ref<CanvasNode[]>([])
const connections = ref<Connection[]>([])
const selectedNodeId = ref<string | null>(null)
const scale = ref(1)
const offsetX = ref(0)
const offsetY = ref(0)

const viewBox = computed(() => {
  return `${offsetX.value} ${offsetY.value} ${800 / scale.value} ${600 / scale.value}`
})

let nextId = 1
let draggingNodeId: string | null = null
let dragStartX = 0
let dragStartY = 0
let nodeStartX = 0
let nodeStartY = 0
let connectingFromNodeId: string | null = null

function onDrop(event: DragEvent) {
  const data = event.dataTransfer?.getData('nodeType')
  if (!data) return
  const type = JSON.parse(data)
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  nodes.value.push({
    id: `node-${nextId++}`,
    type: type.id,
    label: type.label,
    x: event.clientX - rect.left,
    y: event.clientY - rect.top,
  })
}

function selectNode(id: string) {
  selectedNodeId.value = id
}

function onNodeMouseDown(event: MouseEvent, id: string) {
  draggingNodeId = id
  dragStartX = event.clientX
  dragStartY = event.clientY
  const node = nodes.value.find(n => n.id === id)
  if (node) {
    nodeStartX = node.x
    nodeStartY = node.y
  }
}

function onCanvasMouseDown(event: MouseEvent) {
  selectedNodeId.value = null
}

function onCanvasMouseMove(event: MouseEvent) {
  if (draggingNodeId) {
    const node = nodes.value.find(n => n.id === draggingNodeId)
    if (node) {
      node.x = nodeStartX + (event.clientX - dragStartX)
      node.y = nodeStartY + (event.clientY - dragStartY)
    }
  }
}

function onCanvasMouseUp() {
  draggingNodeId = null
  connectingFromNodeId = null
}

function onWheel(event: WheelEvent) {
  const delta = event.deltaY > 0 ? -0.1 : 0.1
  scale.value = Math.max(0.3, Math.min(3, scale.value + delta))
}

function onPortMouseDown(_event: MouseEvent, nodeId: string) {
  connectingFromNodeId = nodeId
}

function onPortMouseUp(_event: MouseEvent, nodeId: string) {
  if (connectingFromNodeId && connectingFromNodeId !== nodeId) {
    const fromNode = nodes.value.find(n => n.id === connectingFromNodeId)
    const toNode = nodes.value.find(n => n.id === nodeId)
    if (fromNode && toNode) {
      connections.value.push({
        id: `conn-${nextId++}`,
        fromNodeId: connectingFromNodeId,
        toNodeId: nodeId,
        x1: fromNode.x + 120,
        y1: fromNode.y + 30,
        x2: toNode.x,
        y2: toNode.y + 30,
      })
    }
  }
  connectingFromNodeId = null
}
</script>

<style scoped>
.designer-canvas {
  flex: 1;
  position: relative;
  overflow: hidden;
  background:
    linear-gradient(90deg, #f0f0f0 1px, transparent 1px),
    linear-gradient(#f0f0f0 1px, transparent 1px);
  background-size: 20px 20px;
}
.connections-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}
.canvas-node {
  position: absolute;
  width: 120px;
  background: #fff;
  border: 2px solid #d9d9d9;
  border-radius: 8px;
  cursor: move;
  user-select: none;
}
.canvas-node.selected {
  border-color: #1890ff;
  box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
}
.node-header {
  padding: 8px 12px;
  font-size: 13px;
  font-weight: 500;
  border-bottom: 1px solid #f0f0f0;
}
.node-type {
  padding: 4px 12px 8px;
  font-size: 11px;
  color: #999;
}
.port {
  position: absolute;
  width: 10px;
  height: 10px;
  background: #d9d9d9;
  border-radius: 50%;
  cursor: crosshair;
}
.port:hover {
  background: #1890ff;
}
.port-out {
  right: -5px;
  top: 50%;
  transform: translateY(-50%);
}
.port-in {
  left: -5px;
  top: 50%;
  transform: translateY(-50%);
}
</style>
