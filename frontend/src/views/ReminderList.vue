<template>
  <div class="reminder-list-container">
    <el-card>
      <template #header>
        <span>ReminderList</span>
      </template>
      <el-empty description="Development in progress - Teaching calendar and reminders" />
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
.reminder-list-container {
  padding: 20px;
}
</style>
