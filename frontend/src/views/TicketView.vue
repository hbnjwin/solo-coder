<template>
  <div class="ticket-view-container">
    <el-card>
      <template #header>
        <span>TicketView</span>
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
.ticket-view-container {
  padding: 20px;
}
</style>
