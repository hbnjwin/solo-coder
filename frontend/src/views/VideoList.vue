<template>
  <div class="video-list-container">
    <el-card>
      <template #header>
        <span>VideoList</span>
      </template>
      <el-empty description="Development in progress - Video subtitle generation" />
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
.video-list-container {
  padding: 20px;
}
</style>
