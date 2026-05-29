<template>
  <div class="student-detail-container">
    <el-card>
      <template #header>
        <span>StudentDetail</span>
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
.student-detail-container {
  padding: 20px;
}
</style>
