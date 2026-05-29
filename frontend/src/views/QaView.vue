<template>
  <div class="qa-view-container">
    <el-card>
      <template #header>
        <span>QaView</span>
      </template>
      <el-empty description="Development in progress - Knowledge base QA citation tracing" />
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
.qa-view-container {
  padding: 20px;
}
</style>
