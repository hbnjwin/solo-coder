<template>
  <div class="quiz-live-container">
    <el-card>
      <template #header>
        <span>QuizLive</span>
      </template>
      <el-empty description="Development in progress - Live classroom quiz" />
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
.quiz-live-container {
  padding: 20px;
}
</style>
