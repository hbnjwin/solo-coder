export interface ContractApprovalPayload {
  contractName: string
  sheetAmount: number
  approver: string
  department: string
  submissionDate: string
  isUrgent: boolean
  notes: string
}

export interface ApprovalResponse {
  requestId: string
  status: 'accepted' | 'rejected' | 'error'
  priority: string
  parsedDate: {
    year: number
    month: number
    day: number
  } | null
  validationErrors: string[]
}

export interface FormState {
  contractName: string
  sheetAmount: number
  approver: string
  department: string
  submissionDate: string
  isUrgent: boolean
  notes: string
}
