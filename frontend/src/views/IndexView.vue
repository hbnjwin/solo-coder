<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Homework AI grading and feedback</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/homework-list')">HomeworkList</el-button>
        <el-button type="primary" @click="$router.push('/feedback-view')">FeedbackView</el-button>
      </el-space>
    </el-card>
  </div>
</template>

<script setup>
</script>

<style scoped>
.index-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}
</style>
