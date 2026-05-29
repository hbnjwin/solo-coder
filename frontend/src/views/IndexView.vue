<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Student chat behavior analysis</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/analysis-view')">AnalysisView</el-button>
        <el-button type="primary" @click="$router.push('/behavior-chart')">BehaviorChart</el-button>
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
