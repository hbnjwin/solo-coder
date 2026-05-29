<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Video subtitle generation</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/video-list')">VideoList</el-button>
        <el-button type="primary" @click="$router.push('/subtitle-edit')">SubtitleEdit</el-button>
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
