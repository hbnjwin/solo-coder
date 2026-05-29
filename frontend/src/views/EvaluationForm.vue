<template>
  <div class="evaluation-form-container">
    <el-card>
      <template #header>
        <span>EvaluationForm</span>
      </template>
      <el-empty description="Development in progress - Course evaluation feedback" />
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
.evaluation-form-container {
  padding: 20px;
}
</style>
