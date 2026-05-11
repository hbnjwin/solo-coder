export interface EscalationItem {
  id: string
  title: string
  approver: string
  submittedAt: string
  status: 'pending' | 'approved' | 'rejected' | 'escalated'
  escalationLevel: number
  resolvedAt: string | null
}

export interface EscalationRule {
  timeoutMinutes: number
  maxLevel: number
  notifyOnEscalate: boolean
}
