<template>
  <div class="order-list-container">
    <el-card>
      <template #header>
        <span>OrderList</span>
      </template>
      <el-empty description="Development in progress - Order refund and tickets" />
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
.order-list-container {
  padding: 20px;
}
</style>
