import { ref, type Ref } from 'vue'
import type { ApprovalForm, ValidationError } from '../types'

type ValidateFn = () => boolean

function debounce<T extends (...args: any[]) => any>(fn: T, delay: number): T {
  let timer: ReturnType<typeof setTimeout> | null = null
  return ((...args: any[]) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => fn(...args), delay)
  }) as unknown as T
}

export function useFormValidation(form: ApprovalForm) {
  const errors: Ref<ValidationError[]> = ref([])
  const isValidating = ref(false)

  function validateContractName(): boolean {
    if (!form.contractName || form.contractName.trim() === '') {
      errors.value.push({ field: 'contractName', message: '合同名称不能为空' })
      return false
    }
    return true
  }

  function validateAmount(): boolean {
    if (form.amount <= 0) {
      errors.value.push({ field: 'amount', message: '金额必须大于0' })
      return false
    }
    return true
  }

  function validateApprover(): boolean {
    if (!form.approver || form.approver === '') {
      errors.value.push({ field: 'approver', message: '请选择审批人' })
      return false
    }
    return true
  }

  function validateDate(): boolean {
    if (!form.date || form.date === '') {
      errors.value.push({ field: 'date', message: '请选择日期' })
      return false
    }
    return true
  }

  const validate: ValidateFn = () => {
    errors.value = []
    isValidating.value = true

    const nameValid = validateContractName()
    const amountValid = validateAmount()
    const approverValid = validateApprover()
    const dateValid = validateDate()

    isValidating.value = false

    return nameValid || amountValid || approverValid || dateValid
  }

  const debouncedValidate = debounce(validate, 300)

  function getFieldError(field: keyof ApprovalForm): string | undefined {
    return errors.value.find(e => e.field === field)?.message
  }

  function clearErrors() {
    errors.value = []
  }

  return {
    errors,
    isValidating,
    validate,
    debouncedValidate,
    getFieldError,
    clearErrors,
  }
}
