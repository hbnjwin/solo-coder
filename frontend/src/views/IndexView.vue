<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Multi-model arena comparison</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/arena-view')">ArenaView</el-button>
        <el-button type="primary" @click="$router.push('/compare-result')">CompareResult</el-button>
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
