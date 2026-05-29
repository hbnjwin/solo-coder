<template>
  <div class="homework-list-container">
    <el-card>
      <template #header>
        <span>HomeworkList</span>
      </template>
      <el-empty description="Development in progress - Homework AI grading and feedback" />
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
.homework-list-container {
  padding: 20px;
}
</style>
