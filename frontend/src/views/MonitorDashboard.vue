<template>
  <div class="monitor-dashboard-container">
    <el-card>
      <template #header>
        <span>MonitorDashboard</span>
      </template>
      <el-empty description="Development in progress - AI workflow debug and monitor" />
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
.monitor-dashboard-container {
  padding: 20px;
}
</style>
