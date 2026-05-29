<template>
  <div class="index-container">
    <el-card>
      <template #header>
        <span>Teaching calendar and reminders</span>
      </template>
      <el-space direction="vertical" :size="12" fill>
        <el-button type="primary" @click="$router.push('/calendar-view')">CalendarView</el-button>
        <el-button type="primary" @click="$router.push('/reminder-list')">ReminderList</el-button>
      </el-space>
    </el-card>
  </div>
</template>

<script setup>
</script>

<style scoped>
.index-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}
</style>
