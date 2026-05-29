<template>
  <div class="report-view-container">
    <el-card>
      <template #header>
        <span>ReportView</span>
      </template>
      <el-empty description="Development in progress - AI exam correction analysis report" />
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
.report-view-container {
  padding: 20px;
}
</style>
