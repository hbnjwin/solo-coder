<template>
  <div class="container">
    <h2>数据导出</h2>
    <div class="progress" v-if="exporting">
      <div class="bar" :style="{ width: progress + '%' }"></div>
      <span>{{ exported }} / {{ total }}</span>
    </div>
    <button @click="startExport" :disabled="exporting">开始导出</button>
    <button @click="cancelExport" v-if="exporting">取消</button>
    <!-- TODO: resume button after network failure -->
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const exporting = ref(false)
const exported = ref(0)
const total = ref(0)
const progress = ref(0)
function startExport() {
  // TODO: implement paginated download with progress
  exporting.value = true
  total.value = 5000
}
function cancelExport() { exporting.value = false }
</script>
<style>
.container { max-width: 600px; margin: 40px auto; padding: 20px; }
.progress { margin: 16px 0; }
.bar { height: 20px; background: #1890ff; border-radius: 4px; transition: width 0.3s; }
button { margin: 4px; padding: 8px 20px; }
</style>