<template>
  <div class="portfolio-detail-container">
    <el-card>
      <template #header>
        <span>PortfolioDetail</span>
      </template>
      <el-empty description="Development in progress - Student AI creation portfolio" />
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
.portfolio-detail-container {
  padding: 20px;
}
</style>
