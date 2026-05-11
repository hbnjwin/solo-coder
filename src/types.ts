export interface Contract {
  id: string
  name: string
  amount: number
  signDate: string
  status: '执行中' | '已完成' | '已终止'
}

export interface ContractDetail {
  period: number
  amount: number
  dueDate: string
  status: '已付' | '未付' | '逾期'
  paidDate?: string
}

export interface LoadingState {
  loading: boolean
  error: string | null
}
