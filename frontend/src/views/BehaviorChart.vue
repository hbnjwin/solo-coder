<template>
  <div class="behavior-chart-container">
    <el-card>
      <template #header>
        <span>BehaviorChart</span>
      </template>
      <el-empty description="Development in progress - Student chat behavior analysis" />
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
.behavior-chart-container {
  padding: 20px;
}
</style>
