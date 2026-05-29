<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Cross-institution resource sharing</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/resource-browser')">ResourceBrowser</el-button>
        <el-button type="primary" @click="$router.push('/share-manage')">ShareManage</el-button>
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
