<template>
  <div class="marketplace-view-container">
    <el-card>
      <template #header>
        <span>MarketplaceView</span>
      </template>
      <el-empty description="Development in progress - AI role marketplace" />
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
.marketplace-view-container {
  padding: 20px;
}
</style>
