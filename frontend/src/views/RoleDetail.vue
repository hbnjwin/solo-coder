<template>
  <div class="role-detail-container">
    <el-card>
      <template #header>
        <span>RoleDetail</span>
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
.role-detail-container {
  padding: 20px;
}
</style>
