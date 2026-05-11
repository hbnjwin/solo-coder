<template>
  <div class="designer">
    <h2>工作流设计器</h2>
    <div class="canvas">
      <div v-for="node in nodes" :key="node.id" class="node" :style="{ left: node.x + 'px', top: node.y + 'px' }">
        {{ node.label }}
      </div>
    </div>
    <!-- TODO: toolbar with draggable node types -->
    <!-- TODO: connection line drawing -->
    <!-- TODO: node deletion with Delete key -->
  </div>
</template>
<script setup lang="ts">
import { reactive } from 'vue'
interface WFNode { id: string; type: string; label: string; x: number; y: number }
const nodes = reactive<WFNode[]>([
  { id: 'start', type: 'start', label: '开始', x: 100, y: 50 },
  { id: 'approval', type: 'approval', label: '审批', x: 100, y: 150 },
  { id: 'end', type: 'end', label: '结束', x: 100, y: 250 },
])
</script>
<style>
.designer { padding: 20px; }
.canvas { position: relative; width: 800px; height: 400px; border: 1px solid #ddd; background: #fafafa; }
.node { position: absolute; padding: 10px 20px; background: #1890ff; color: white; border-radius: 4px; cursor: pointer; min-width: 80px; text-align: center; }
</style>