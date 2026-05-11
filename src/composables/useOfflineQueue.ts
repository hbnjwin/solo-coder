import { ref, computed } from 'vue'
import type { QueueItem, ApprovalRequest, SyncStatus } from '../types'

const STORAGE_KEY = 'offline_queue'
const MAX_QUEUE_SIZE = 50

const queue = ref<QueueItem[]>(loadFromStorage())

function loadFromStorage(): QueueItem[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return []
    return JSON.parse(raw) as QueueItem[]
  } catch {
    return []
  }
}

function persistToStorage() {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(queue.value))
  } catch (e) {
    console.warn('Failed to persist queue to localStorage:', e)
  }
}

function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`
}

export function useOfflineQueue() {
  const pendingItems = computed(() =>
    queue.value.filter(item => item.status === 'pending' || item.status === 'failed')
  )

  const queueSize = computed(() => queue.value.length)

  function enqueue(payload: ApprovalRequest): QueueItem {
    const item: QueueItem = {
      id: generateId(),
      payload,
      timestamp: Date.now(),
      status: 'pending',
      retryCount: 0
    }

    if (queue.value.length >= MAX_QUEUE_SIZE) {
      console.warn(`Queue at capacity (${MAX_QUEUE_SIZE}), oldest item will be dropped`)
    }

    queue.value.push(item)
    persistToStorage()
    return item
  }

  function updateStatus(id: string, status: SyncStatus) {
    const item = queue.value.find(i => i.id === id)
    if (item) {
      item.status = status
      if (status === 'failed') {
        item.retryCount++
      }
      persistToStorage()
    }
  }

  function clear() {
    queue.value = []
    persistToStorage()
  }

  return {
    queue,
    pendingItems,
    queueSize,
    enqueue,
    updateStatus,
    clear
  }
}
