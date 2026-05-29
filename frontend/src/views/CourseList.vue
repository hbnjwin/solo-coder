<template>
  <div class="course-list-container">
    <el-card>
      <template #header>
        <span>CourseList</span>
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
.course-list-container {
  padding: 20px;
}
</style>
