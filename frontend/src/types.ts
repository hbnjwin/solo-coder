export interface ExportRecord {
  id: number
  entityName: string
  amount: number
  status: 'active' | 'archived' | 'pending'
  owner: string
  createdAt: number
}

export interface ExportPageResponse {
  records: ExportRecord[]
  nextCursor: string | null
  totalPages: number
  currentPage: number
}

export interface ExportState {
  status: 'idle' | 'exporting' | 'paused' | 'error' | 'complete'
  pagesCompleted: number
  totalPages: number
  totalRecords: number
  lastSuccessfulPage: number
  errorMessage: string | null
}

export interface ExportHistoryEntry {
  id: string
  startedAt: number
  completedAt: number | null
  recordCount: number
  status: 'complete' | 'failed' | 'cancelled'
}
