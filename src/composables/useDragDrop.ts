import { ref } from 'vue'
import type { NodeType, Position } from '../types'
import { GRID_SIZE } from '../types'
import { useDesigner } from './useDesigner'

const isDragging = ref(false)
const dragNodeType = ref<NodeType | null>(null)
const dragLabel = ref('')

function snapToGrid(value: number): number {
  return Math.round(value / GRID_SIZE) * GRID_SIZE
}

export function useDragDrop() {
  const { addNode } = useDesigner()

  function onDragStart(type: NodeType, label: string): void {
    isDragging.value = true
    dragNodeType.value = type
    dragLabel.value = label
  }

  function onDragOver(event: DragEvent): void {
    event.preventDefault()
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'copy'
    }
  }

  function onDrop(event: DragEvent): void {
    event.preventDefault()
    if (!dragNodeType.value) return

    const x = snapToGrid(event.clientX)
    const y = snapToGrid(event.clientY)

    const position: Position = { x, y }

    addNode(dragNodeType.value, dragLabel.value, position)

    isDragging.value = false
    dragNodeType.value = null
    dragLabel.value = ''
  }

  function onDragEnd(): void {
    isDragging.value = false
    dragNodeType.value = null
    dragLabel.value = ''
  }

  return {
    isDragging,
    dragNodeType,
    onDragStart,
    onDragOver,
    onDrop,
    onDragEnd,
  }
}
