<template>
  <div class="prompt-editor-container">
    <el-card>
      <template #header>
        <span>PromptEditor</span>
      </template>
      <el-empty description="Development in progress - Chat prompt template library" />
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
.prompt-editor-container {
  padding: 20px;
}
</style>
