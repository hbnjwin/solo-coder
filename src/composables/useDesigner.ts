import { reactive, ref } from 'vue'
import type { WorkflowNode, Connection, NodeType, Position } from '../types'

const nodes = reactive<WorkflowNode[]>([])
const connections = reactive<Connection[]>([])
const selectedNodeId = ref<string | null>(null)

let nodeCounter = 0

function generateId(): string {
  return `node_${Date.now()}_${++nodeCounter}`
}

export function useDesigner() {
  function addNode(type: NodeType, label: string, position: Position): WorkflowNode {
    const node: WorkflowNode = {
      id: generateId(),
      type,
      label,
      position: { ...position },
    }
    nodes.push(node)
    return node
  }

  function deleteNode(nodeId: string): void {
    const index = nodes.findIndex(n => n.id === nodeId)
    if (index === -1) return
    nodes.splice(index, 1)
    if (selectedNodeId.value === nodeId) {
      selectedNodeId.value = null
    }
  }

  function updateNodePosition(nodeId: string, position: Position): void {
    const node = nodes.find(n => n.id === nodeId)
    if (node) {
      node.position.x = position.x
      node.position.y = position.y
    }
  }

  function selectNode(nodeId: string | null): void {
    selectedNodeId.value = nodeId
  }

  function addConnection(sourceId: string, targetId: string): Connection | null {
    if (sourceId === targetId) return null

    const sourceExists = nodes.some(n => n.id === sourceId)
    const targetExists = nodes.some(n => n.id === targetId)
    if (!sourceExists || !targetExists) return null

    const connection: Connection = {
      id: `conn_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
      sourceId,
      targetId,
    }
    connections.push(connection)
    return connection
  }

  function deleteConnection(connectionId: string): void {
    const index = connections.findIndex(c => c.id === connectionId)
    if (index !== -1) {
      connections.splice(index, 1)
    }
  }

  function getNodeById(nodeId: string): WorkflowNode | undefined {
    return nodes.find(n => n.id === nodeId)
  }

  return {
    nodes,
    connections,
    selectedNodeId,
    addNode,
    deleteNode,
    updateNodePosition,
    selectNode,
    addConnection,
    deleteConnection,
    getNodeById,
  }
}
