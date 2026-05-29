<template>
  <div class="compare-result-container">
    <el-card>
      <template #header>
        <span>CompareResult</span>
      </template>
      <el-empty description="Development in progress - Multi-model arena comparison" />
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
.compare-result-container {
  padding: 20px;
}
</style>
