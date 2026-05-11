import { ref, onMounted } from 'vue'
import type { Contract } from '../types'
import { fetchContracts } from '../api/mockData'

export function useContractList() {
  const contracts = ref<Contract[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadContracts() {
    loading.value = true
    error.value = null
    try {
      contracts.value = await fetchContracts()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载合同列表失败'
    } finally {
      loading.value = false
    }
  }

  onMounted(() => {
    loadContracts()
  })

  return {
    contracts,
    loading,
    error,
    reload: loadContracts,
  }
}
