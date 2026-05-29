<template>
  <div class="audit-flow-container">
    <el-card>
      <template #header>
        <span>AuditFlow</span>
      </template>
      <el-empty description="Development in progress - Competition audit workflow" />
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
.audit-flow-container {
  padding: 20px;
}
</style>
