<template>
  <div class="progress-dashboard-container">
    <el-card>
      <template #header>
        <span>ProgressDashboard</span>
      </template>
      <el-empty description="Development in progress - Course learning progress tracking" />
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
.progress-dashboard-container {
  padding: 20px;
}
</style>
