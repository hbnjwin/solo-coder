<template>
  <svg
    v-if="sourcePos && targetPos"
    class="connection-line"
    :style="svgStyle"
  >
    <line
      :x1="x1"
      :y1="y1"
      :x2="x2"
      :y2="y2"
      stroke="#999"
      stroke-width="2"
      marker-end="url(#arrowhead)"
    />
    <defs>
      <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
        <polygon points="0 0, 10 3.5, 0 7" fill="#999" />
      </marker>
    </defs>
  </svg>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Position } from '../types'

const props = defineProps<{
  sourcePos: Position | undefined
  targetPos: Position | undefined
}>()

const NODE_WIDTH = 100
const NODE_HEIGHT = 36

const x1 = computed(() => props.sourcePos!.x + NODE_WIDTH / 2)
const y1 = computed(() => props.sourcePos!.y + NODE_HEIGHT)
const x2 = computed(() => props.targetPos!.x + NODE_WIDTH / 2)
const y2 = computed(() => props.targetPos!.y)

const svgStyle = computed(() => ({
  position: 'absolute' as const,
  top: '0',
  left: '0',
  width: '100%',
  height: '100%',
  pointerEvents: 'none' as const,
}))
</script>

<style scoped>
.connection-line {
  overflow: visible;
}
</style>
