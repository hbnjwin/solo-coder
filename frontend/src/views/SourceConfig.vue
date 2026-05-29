<template>
  <div class="source-config-container">
    <el-card>
      <template #header>
        <span>SourceConfig</span>
      </template>
      <el-empty description="Development in progress - Knowledge base auto-sync" />
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
.source-config-container {
  padding: 20px;
}
</style>
