<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>I18n translation management</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/translation-editor')">TranslationEditor</el-button>
        <el-button type="primary" @click="$router.push('/language-config')">LanguageConfig</el-button>
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
