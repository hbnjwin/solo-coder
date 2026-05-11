import { ref } from 'vue'
import { useOfflineQueue } from './useOfflineQueue'
import { useNetworkStatus } from './useNetworkStatus'
import type { QueueItem } from '../types'

async function sendToServer(item: QueueItem): Promise<boolean> {
  await new Promise(resolve => setTimeout(resolve, 300 + Math.random() * 700))
  return Math.random() > 0.1
}

export function useSyncEngine() {
  const { queue, updateStatus } = useOfflineQueue()
  const { isOnline } = useNetworkStatus()
  const isSyncing = ref(false)
  const syncProgress = ref(0)
  const syncTotal = ref(0)

  // FIXME: concurrent sync calls may cause duplicate submissions
  async function syncAll() {
    if (isSyncing.value) return
    if (!isOnline.value) return

    const itemsToSync = queue.value.filter(
      i => i.status === 'pending' || i.status === 'failed'
    )

    if (itemsToSync.length === 0) return

    isSyncing.value = true
    syncProgress.value = 0
    syncTotal.value = itemsToSync.length

    for (const item of itemsToSync) {
      if (!isOnline.value) break

      updateStatus(item.id, 'syncing')
      const success = await sendToServer(item)

      if (success) {
        updateStatus(item.id, 'synced')
      } else {
        updateStatus(item.id, 'failed')
      }

      syncProgress.value++
    }

    isSyncing.value = false
  }

  return {
    isSyncing,
    syncProgress,
    syncTotal,
    syncAll
  }
}
