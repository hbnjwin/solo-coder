<template>
  <div class="mindmap-editor-container">
    <el-card>
      <template #header>
        <span>MindmapEditor</span>
      </template>
      <el-empty description="Development in progress - Mindmap collaboration and expansion" />
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const loading = ref(false)
const dataList = ref([])

onMounted(() => {
  // TODO: fetch data from API
})
</script>

<style scoped>
.mindmap-editor-container {
  padding: 20px;
}
</style>
