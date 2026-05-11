export interface ApprovalRequest {
  contractId: string
  comment: string
  approver: string
}

export type SyncStatus = 'pending' | 'syncing' | 'synced' | 'failed'

export interface QueueItem {
  id: string
  payload: ApprovalRequest
  timestamp: number
  status: SyncStatus
  retryCount: number
}
