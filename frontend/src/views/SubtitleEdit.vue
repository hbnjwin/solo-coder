<template>
  <div class="subtitle-edit-container">
    <el-card>
      <template #header>
        <span>SubtitleEdit</span>
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
.subtitle-edit-container {
  padding: 20px;
}
</style>
