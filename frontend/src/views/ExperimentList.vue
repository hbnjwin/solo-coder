<template>
  <div class="experiment-list-container">
    <el-card>
      <template #header>
        <span>ExperimentList</span>
      </template>
      <el-empty description="Development in progress - ML experiment management" />
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
.experiment-list-container {
  padding: 20px;
}
</style>
