<template>
  <div class="property-panel">
    <h3>属性面板</h3>
    <div v-if="selectedNode" class="property-form">
      <div class="form-item">
        <label>节点名称</label>
        <input v-model="selectedNode.label" />
      </div>
      <div class="form-item">
        <label>节点类型</label>
        <span>{{ selectedNode.type }}</span>
      </div>
      <div v-if="selectedNode.type === 'approval'" class="form-item">
        <label>审批人</label>
        <input placeholder="输入审批人" />
      </div>
      <div v-if="selectedNode.type === 'approval'" class="form-item">
        <label>超时时间(小时)</label>
        <input type="number" placeholder="24" />
      </div>
      <div v-if="selectedNode.type === 'condition'" class="form-item">
        <label>条件表达式</label>
        <input placeholder="amount > 10000" />
      </div>
      <div v-if="selectedNode.type === 'countersign'" class="form-item">
        <label>通过比例(%)</label>
        <input type="number" placeholder="60" />
      </div>
      <div v-if="selectedNode.type === 'cc'" class="form-item">
        <label>抄送人</label>
        <input placeholder="输入抄送人" />
      </div>
    </div>
    <div v-else class="empty-hint">
      请选中画布上的节点
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  nodes: Array<{ id: string; type: string; label: string; x: number; y: number }>
  selectedNodeId: string | null
}>()

const selectedNode = computed(() => {
  if (!props.selectedNodeId) return null
  return props.nodes.find(n => n.id === props.selectedNodeId) || null
})
</script>

<style scoped>
.property-panel {
  width: 260px;
  background: #fff;
  border-left: 1px solid #e0e0e0;
  padding: 16px;
}
.property-panel h3 {
  font-size: 14px;
  color: #333;
  margin-bottom: 12px;
}
.form-item {
  margin-bottom: 12px;
}
.form-item label {
  display: block;
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}
.form-item input {
  width: 100%;
  padding: 6px 8px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
}
.empty-hint {
  color: #999;
  font-size: 13px;
  text-align: center;
  margin-top: 40px;
}
</style>
