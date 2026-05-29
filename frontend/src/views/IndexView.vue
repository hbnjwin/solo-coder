<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>AI role marketplace</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/marketplace-view')">MarketplaceView</el-button>
        <el-button type="primary" @click="$router.push('/role-detail')">RoleDetail</el-button>
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
