import type { ContractApprovalPayload, ApprovalResponse } from './types'

const API_BASE = '/api'

export async function submitApproval(payload: ContractApprovalPayload): Promise<ApprovalResponse> {
  const body = buildRequestBody(payload)

  const response = await fetch(`${API_BASE}/approval`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  })

  if (!response.ok) {
    throw new Error(`API request failed: ${response.status} ${response.statusText}`)
  }

  return response.json()
}

function buildRequestBody(payload: ContractApprovalPayload): Record<string, unknown> {
  return {
    contractName: payload.contractName,
    sheetAmount: payload.sheetAmount,
    approver: payload.approver,
    department: payload.department,
    submissionDate: formatDateForApi(payload.submissionDate),
    isUrgent: payload.isUrgent,
    notes: payload.notes || null,
  }
}

function formatDateForApi(localDatetime: string): string {
  if (!localDatetime) return ''
  const date = new Date(localDatetime)
  return date.toISOString()
}
