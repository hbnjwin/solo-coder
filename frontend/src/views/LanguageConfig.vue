<template>
  <div class="language-config-container">
    <el-card>
      <template #header>
        <span>LanguageConfig</span>
      </template>
      <el-empty description="Development in progress - I18n translation management" />
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
.language-config-container {
  padding: 20px;
}
</style>
