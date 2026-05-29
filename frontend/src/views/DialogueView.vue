<template>
  <div class="dialogue-view-container">
    <el-card>
      <template #header>
        <span>DialogueView</span>
      </template>
      <el-empty description="Development in progress - Voice dialogue scoring" />
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
.dialogue-view-container {
  padding: 20px;
}
</style>
