export type NodeType = 'start' | 'end' | 'approval' | 'condition' | 'action'

export interface Position {
  x: number
  y: number
}

export interface WorkflowNode {
  id: string
  type: NodeType
  label: string
  position: Position
}

export interface Connection {
  id: string
  sourceId: string
  targetId: string
}

export interface NodeTypeConfig {
  type: NodeType
  label: string
  color: string
}

export const NODE_TYPES: NodeTypeConfig[] = [
  { type: 'start', label: '开始', color: '#52c41a' },
  { type: 'end', label: '结束', color: '#ff4d4f' },
  { type: 'approval', label: '审批', color: '#1890ff' },
  { type: 'condition', label: '条件', color: '#faad14' },
  { type: 'action', label: '动作', color: '#722ed1' },
]

export const GRID_SIZE = 20
