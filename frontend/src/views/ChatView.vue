<template>
  <div class="chat-view-container">
    <el-card>
      <template #header>
        <span>ChatView</span>
      </template>
      <el-empty description="Development in progress - AI chat context memory compression" />
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
.chat-view-container {
  padding: 20px;
}
</style>
