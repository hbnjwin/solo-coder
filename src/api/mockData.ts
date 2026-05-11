import type { Contract, ContractDetail } from '../types'

const contracts: Contract[] = [
  { id: 'C001', name: '华东区域供货合同', amount: 500000, signDate: '2024-01-15', status: '执行中' },
  { id: 'C002', name: '设备采购框架协议', amount: 2000000, signDate: '2024-03-01', status: '执行中' },
  { id: 'C003', name: '年度维保服务合同', amount: 800000, signDate: '2023-11-20', status: '已完成' },
  { id: 'C004', name: '软件许可授权合同', amount: 350000, signDate: '2024-06-10', status: '执行中' },
  { id: 'C005', name: '物流运输服务协议', amount: 1200000, signDate: '2024-02-28', status: '已终止' },
]

const detailMap: Record<string, ContractDetail[]> = {
  C001: [
    { period: 1, amount: 150000, dueDate: '2024-02-15', status: '已付', paidDate: '2024-02-14' },
    { period: 2, amount: 150000, dueDate: '2024-05-15', status: '已付', paidDate: '2024-05-20' },
    { period: 3, amount: 200000, dueDate: '2024-08-15', status: '未付' },
  ],
  C002: [
    { period: 1, amount: 600000, dueDate: '2024-04-01', status: '已付', paidDate: '2024-03-30' },
    { period: 2, amount: 700000, dueDate: '2024-07-01', status: '逾期' },
    { period: 3, amount: 700000, dueDate: '2024-10-01', status: '未付' },
  ],
  C003: [
    { period: 1, amount: 400000, dueDate: '2024-01-20', status: '已付', paidDate: '2024-01-18' },
    { period: 2, amount: 400000, dueDate: '2024-06-20', status: '已付', paidDate: '2024-06-19' },
  ],
  C004: [
    { period: 1, amount: 350000, dueDate: '2024-07-10', status: '未付' },
  ],
  C005: [
    { period: 1, amount: 400000, dueDate: '2024-03-28', status: '已付', paidDate: '2024-03-25' },
    { period: 2, amount: 400000, dueDate: '2024-06-28', status: '已付', paidDate: '2024-06-30' },
    { period: 3, amount: 400000, dueDate: '2024-09-28', status: '未付' },
  ],
}

// FIXME: random delays may cause test flakiness
function randomDelay(): number {
  return 300 + Math.random() * 700
}

export function fetchContracts(): Promise<Contract[]> {
  return new Promise((resolve) => {
    setTimeout(() => resolve([...contracts]), randomDelay())
  })
}

export function fetchContractDetail(contractId: string): Promise<ContractDetail[]> {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      const details = detailMap[contractId]
      if (details) {
        resolve([...details])
      } else {
        reject(new Error(`合同 ${contractId} 不存在`))
      }
    }, randomDelay())
  })
}
