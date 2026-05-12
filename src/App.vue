<template>
  <div class="editor-layout">
    <NodePanel />
    <DesignerCanvas :nodes="nodes"
                    :selected-node-id="selectedNodeId"
                    @add-node="onAddNode"
                    @select-node="onSelectNode"
                    @move-node="onMoveNode"
                    @add-connection="onAddConnection" />
    <PropertyPanel :nodes="nodes"
                   :selected-node-id="selectedNodeId"
                   @update-node="onUpdateNode" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NodePanel from './components/NodePanel.vue'
import DesignerCanvas from './components/DesignerCanvas.vue'
import PropertyPanel from './components/PropertyPanel.vue'

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
}

const nodes = ref<CanvasNode[]>([])
const connections = ref<Connection[]>([])
const selectedNodeId = ref<string | null>(null)

let nextId = 1

function onAddNode(type: { id: string; label: string; x: number; y: number }) {
  nodes.value.push({
    id: `node-${nextId++}`,
    type: type.id,
    label: type.label,
    x: type.x,
    y: type.y,
  })
}

function onSelectNode(id: string | null) {
  selectedNodeId.value = id
}

function onMoveNode(payload: { id: string; x: number; y: number }) {
  const node = nodes.value.find((n) => n.id === payload.id)
  if (node) {
    node.x = payload.x
    node.y = payload.y
  }
}

function onAddConnection(payload: { fromNodeId: string; toNodeId: string }) {
  connections.value.push({
    id: `conn-${nextId++}`,
    fromNodeId: payload.fromNodeId,
    toNodeId: payload.toNodeId,
  })
}

function onUpdateNode(payload: { id: string; label: string }) {
  const node = nodes.value.find((n) => n.id === payload.id)
  if (node) {
    node.label = payload.label
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f0f2f5;
}
.editor-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
}
</style>
