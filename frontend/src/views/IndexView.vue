<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Mindmap collaboration and expansion</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/mindmap-editor')">MindmapEditor</el-button>
        <el-button type="primary" @click="$router.push('/collab-view')">CollabView</el-button>
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
