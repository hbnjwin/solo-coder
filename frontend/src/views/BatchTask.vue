<template>
  <div class="batch-task-container">
    <el-card>
      <template #header>
        <span>BatchTask</span>
      </template>
      <el-empty description="Development in progress - Image style transfer batch processing" />
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
.batch-task-container {
  padding: 20px;
}
</style>
