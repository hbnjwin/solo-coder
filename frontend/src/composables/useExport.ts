import { ref, computed } from 'vue'
import type { ExportState, ExportPageResponse } from '../types'

const API_BASE = '/api/export'

const state = ref<ExportState>({
  status: 'idle',
  pagesCompleted: 0,
  totalPages: 0,
  totalRecords: 0,
  lastSuccessfulPage: 0,
  errorMessage: null,
})

const progress = computed(() => {
  if (state.value.totalPages === 0) return 0
  return Math.round((state.value.pagesCompleted / state.value.totalPages) * 100)
})

async function fetchPage(page: number): Promise<ExportPageResponse> {
  const res = await fetch(`${API_BASE}/page?page=${page}`)
  if (!res.ok) throw new Error(`Export failed at page ${page}: ${res.statusText}`)
  return res.json()
}

async function startExport() {
  state.value.status = 'exporting'
  state.value.pagesCompleted = 0
  state.value.errorMessage = null

  const initial = await fetchPage(0)
  state.value.totalPages = initial.totalPages
  state.value.totalRecords = initial.records.length

  let currentPage = 0

  while (currentPage < state.value.totalPages) {
    if (state.value.status !== 'exporting') break

    try {
      const response = await fetchPage(currentPage)
      state.value.totalRecords += response.records.length
      currentPage++
      state.value.pagesCompleted = currentPage
      state.value.lastSuccessfulPage = currentPage
    } catch (err: any) {
      state.value.status = 'error'
      state.value.errorMessage = err.message
      return
    }
  }

  if (state.value.status === 'exporting') {
    state.value.status = 'complete'
  }
}

async function resume() {
  state.value.errorMessage = null
  await startExport()
}

function pause() {
  if (state.value.status === 'exporting') {
    state.value.status = 'paused'
  }
}

function reset() {
  state.value = {
    status: 'idle',
    pagesCompleted: 0,
    totalPages: 0,
    totalRecords: 0,
    lastSuccessfulPage: 0,
    errorMessage: null,
  }
}

export function useExport() {
  return {
    state,
    progress,
    startExport,
    resume,
    pause,
    reset,
  }
}
