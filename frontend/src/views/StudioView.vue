<template>
  <div class="studio-view-container">
    <el-card>
      <template #header>
        <span>StudioView</span>
      </template>
      <el-empty description="Development in progress - Music collaboration and remix" />
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
.studio-view-container {
  padding: 20px;
}
</style>
