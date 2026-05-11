import { computed } from 'vue'
import { APPROVER_OPTIONS, AMOUNT_THRESHOLD_GENERAL_MANAGER } from '../types'
import type { ApprovalForm, ApproverOption } from '../types'

export function useApproverRouting(form: ApprovalForm) {
  const availableApprovers = computed<ApproverOption[]>(() => {
    const options = APPROVER_OPTIONS.filter(option => {
      if (option.value === 'general_manager') {
        return form.amount >= AMOUNT_THRESHOLD_GENERAL_MANAGER
      }
      if (option.value === 'vice_president') {
        return form.amount >= option.minAmount
      }
      return true
    })
    return options
  })

  const recommendedApprover = computed<string>(() => {
    if (form.amount >= AMOUNT_THRESHOLD_GENERAL_MANAGER) {
      return 'general_manager'
    }
    if (form.amount >= 500000) {
      return 'vice_president'
    }
    return 'department_manager'
  })

  const isHighValueContract = computed(() => {
    return form.amount >= AMOUNT_THRESHOLD_GENERAL_MANAGER
  })

  function getApproverLabel(value: string): string {
    return APPROVER_OPTIONS.find(o => o.value === value)?.label ?? '未知'
  }

  return {
    availableApprovers,
    recommendedApprover,
    isHighValueContract,
    getApproverLabel,
  }
}
