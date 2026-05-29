<template>
  <div class="style-gallery-container">
    <el-card>
      <template #header>
        <span>StyleGallery</span>
      </template>
      <el-empty description="Development in progress - Image style transfer batch processing" />
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
.style-gallery-container {
  padding: 20px;
}
</style>
