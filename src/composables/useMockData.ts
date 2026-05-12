import { ref } from 'vue'

interface Stats {
  totalCount: number
  avgDuration: number
  pendingCount: number
  timeoutCount: number
}

interface TrendItem {
  hour: string
  count: number
}

interface DeptItem {
  dept: string
  rate: number
}

interface TypeItem {
  name: string
  value: number
}

export function useMockData() {
  const stats = ref<Stats>({ totalCount: 0, avgDuration: 0, pendingCount: 0, timeoutCount: 0 })
  const trendData = ref<TrendItem[]>([])
  const deptData = ref<DeptItem[]>([])
  const typeData = ref<TypeItem[]>([])
  const heatmapData = ref<Array<Array<number | string>>>([])

  function rand(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1)) + min
  }

  function refresh() {
    stats.value = {
      totalCount: rand(200, 800),
      avgDuration: rand(15, 90),
      pendingCount: rand(10, 60),
      timeoutCount: rand(0, 15),
    }

    trendData.value = Array.from({ length: 24 }, (_, i) => ({
      hour: `${i}:00`,
      count: rand(5, 50),
    }))

    deptData.value = [
      { dept: '财务部', rate: rand(70, 98) },
      { dept: '人事部', rate: rand(65, 95) },
      { dept: '技术部', rate: rand(75, 99) },
      { dept: '市场部', rate: rand(60, 92) },
      { dept: '运营部', rate: rand(68, 96) },
    ]

    typeData.value = [
      { name: '合同审批', value: rand(30, 100) },
      { name: '报销审批', value: rand(20, 80) },
      { name: '请假审批', value: rand(10, 60) },
      { name: '采购审批', value: rand(15, 50) },
    ]

    const depts = ['财务部', '人事部', '技术部', '市场部', '运营部']
    heatmapData.value = []
    for (let h = 0; h < 24; h++) {
      for (const dept of depts) {
        heatmapData.value.push([h, dept, rand(10, 120)])
      }
    }
  }

  refresh()

  return { stats, trendData, deptData, typeData, heatmapData, refresh }
}
