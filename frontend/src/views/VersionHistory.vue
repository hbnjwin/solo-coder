<template>
  <div class="version-history-container">
    <el-card>
      <template #header>
        <span>VersionHistory</span>
      </template>
      <el-empty description="Development in progress - AI writing collaboration with versioning" />
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
.version-history-container {
  padding: 20px;
}
</style>
