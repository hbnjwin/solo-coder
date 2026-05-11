export interface ApprovalForm {
  contractName: string
  amount: number
  approver: string
  date: string
}

export interface ValidationError {
  field: keyof ApprovalForm
  message: string
}

export interface ApproverOption {
  value: string
  label: string
  minAmount: number
}

export const APPROVER_OPTIONS: ApproverOption[] = [
  { value: 'department_manager', label: '部门经理', minAmount: 0 },
  { value: 'vice_president', label: '副总裁', minAmount: 500000 },
  { value: 'general_manager', label: '总经理', minAmount: 1000000 },
]

export const AMOUNT_THRESHOLD_GENERAL_MANAGER = 10000000

export type FormFieldName = keyof ApprovalForm
