<template>
  <div class="app-container">
    <h2>合同管理系统</h2>
    <div class="layout">
      <ContractList
        :contracts="contracts"
        :selected-id="selectedId"
        :loading="listLoading"
        @select="handleSelect"
      />
      <ContractDetail :selected-id="selectedId" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ContractList from './components/ContractList.vue'
import ContractDetail from './components/ContractDetail.vue'
import { useContractList } from './composables/useContractList'

const { contracts, loading: listLoading } = useContractList()
const selectedId = ref('')

function handleSelect(id: string) {
  selectedId.value = id
}

onMounted(() => {
  selectedId.value = 'C001'
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
}
.app-container {
  max-width: 1100px;
  margin: 0 auto;
  padding: 24px;
}
h2 {
  font-size: 18px;
  color: #222;
  margin-bottom: 16px;
}
.layout {
  display: flex;
  gap: 24px;
  background: #fff;
  border-radius: 6px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}
</style>
