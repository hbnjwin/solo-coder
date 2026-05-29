<template>
  <div class="arena-view-container">
    <el-card>
      <template #header>
        <span>ArenaView</span>
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
.arena-view-container {
  padding: 20px;
}
</style>
