<template>
  <div class="tenant-config-container">
    <el-card>
      <template #header>
        <span>TenantConfig</span>
      </template>
      <el-empty description="Development in progress - Multi-tenant data isolation" />
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
.tenant-config-container {
  padding: 20px;
}
</style>
