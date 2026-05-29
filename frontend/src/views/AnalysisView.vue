<template>
  <div class="analysis-view-container">
    <el-card>
      <template #header>
        <span>AnalysisView</span>
      </template>
      <el-empty description="Development in progress - Student chat behavior analysis" />
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
.analysis-view-container {
  padding: 20px;
}
</style>
