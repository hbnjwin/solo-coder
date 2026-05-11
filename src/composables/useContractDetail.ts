import { ref } from 'vue'
import type { ContractDetail } from '../types'
import { fetchContractDetail } from '../api/mockData'

export function useContractDetail() {
  const details = ref<ContractDetail[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const pendingTimer = ref<ReturnType<typeof setTimeout> | null>(null)

  function loadDetail(contractId: string) {
    if (!contractId) {
      details.value = []
      return
    }

    loading.value = true
    error.value = null

    // Store the timer reference for potential cleanup
    pendingTimer.value = setTimeout(async () => {
      try {
        const result = await fetchContractDetail(contractId)
        details.value = result
      } catch (e) {
        error.value = e instanceof Error ? e.message : '加载明细失败'
        details.value = []
      } finally {
        loading.value = false
      }
    }, 0)
  }

  function clearPending() {
    if (pendingTimer.value !== null) {
      pendingTimer.value = null
    }
  }

  return {
    details,
    loading,
    error,
    loadDetail,
    clearPending,
  }
}
