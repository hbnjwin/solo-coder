<template>
  <div class="resource-browser-container">
    <el-card>
      <template #header>
        <span>ResourceBrowser</span>
      </template>
      <el-empty description="Development in progress - Cross-institution resource sharing" />
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
.resource-browser-container {
  padding: 20px;
}
</style>
