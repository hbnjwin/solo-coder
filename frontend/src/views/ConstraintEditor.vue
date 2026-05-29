<template>
  <div class="constraint-editor-container">
    <el-card>
      <template #header>
        <span>ConstraintEditor</span>
      </template>
      <el-empty description="Development in progress - Smart timetabling" />
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
.constraint-editor-container {
  padding: 20px;
}
</style>
