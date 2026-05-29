<template>
  <div class="analytics-dashboard-container">
    <el-card>
      <template #header>
        <span>AnalyticsDashboard</span>
      </template>
      <el-empty description="Development in progress - Student analytics dashboard" />
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
.analytics-dashboard-container {
  padding: 20px;
}
</style>
