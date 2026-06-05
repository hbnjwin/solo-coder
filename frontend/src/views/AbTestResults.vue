<template>
  <div class="ab-test-results"
       v-loading="loading">
    <div class="header">
      <el-button @click="goBack">
        <el-icon>
          <ArrowLeft />
        </el-icon>
        返回
      </el-button>
      <h2>{{ results?.test_name }} - 测试结果</h2>
      <el-tag :type="statusType"
              size="large">{{ statusLabel }}</el-tag>
    </div>

    <el-row :gutter="20"
            class="summary-cards">
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-item">
            <div class="stat-label">总调用次数</div>
            <div class="stat-value">{{ results?.total_calls || 0 }}</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-item">
            <div class="stat-label">测试变体</div>
            <div class="stat-value">{{ results?.variant_results?.length || 0 }} 个</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-item">
            <div class="stat-label">统计显著性</div>
            <div class="stat-value">
              <span v-if="results?.statistical_analysis"
                    :class="{ 'text-success': results.statistical_analysis.is_significant, 'text-warning': !results.statistical_analysis.is_significant }">
                {{ results.statistical_analysis.is_significant ? '显著' : '不显著' }}
              </span>
              <span v-else
                    class="text-muted">-</span>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover"
                 class="recommended">
          <div class="stat-item">
            <div class="stat-label">
              <el-icon>
                <StarFilled />
              </el-icon>
              推荐变体
            </div>
            <div class="stat-value text-primary">
              {{ results?.recommended_variant_name || '暂无' }}
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-card class="chart-card"
             v-if="results?.variant_results?.length">
      <template #header>
        <div class="card-header">
          <span>各变体指标对比</span>
          <el-tag size="small"
                  type="info">点击图例可切换显示</el-tag>
        </div>
      </template>
      <div ref="chartRef"
           class="chart"></div>
    </el-card>

    <el-card class="chart-card"
             v-if="results?.variant_results?.length">
      <template #header>
        <span>满意度对比</span>
      </template>
      <div ref="satisfactionChartRef"
           class="chart"></div>
    </el-card>

    <el-card v-if="results?.statistical_analysis">
      <template #header>
        <span>统计显著性分析</span>
      </template>
      <div class="stats-analysis">
        <el-row :gutter="20">
          <el-col :span="8">
            <div class="stats-item">
              <div class="stats-label">卡方值</div>
              <div class="stats-value">{{ results.statistical_analysis.chi_square?.toFixed(4) || 'N/A' }}</div>
            </div>
          </el-col>
          <el-col :span="8">
            <div class="stats-item">
              <div class="stats-label">自由度</div>
              <div class="stats-value">{{ results.statistical_analysis.degrees_of_freedom || 'N/A' }}</div>
            </div>
          </el-col>
          <el-col :span="8">
            <div class="stats-item">
              <div class="stats-label">P值</div>
              <div class="stats-value"
                   :class="{ 'text-success': results.statistical_analysis.p_value < 0.05 }">
                {{ results.statistical_analysis.p_value?.toFixed(6) || 'N/A' }}
              </div>
            </div>
          </el-col>
        </el-row>
        <el-alert :title="significanceMessage"
                  :type="results.statistical_analysis.is_significant ? 'success' : 'info'"
                  :closable="false"
                  style="margin-top: 20px;">
          <template #default>
            <p>显著性水平：α = {{ results.statistical_analysis.significance_level }}</p>
            <p>当 P值 < 0.05
                时，认为各变体之间存在统计学显著差异。</p>
          </template>
        </el-alert>
      </div>
    </el-card>

    <el-card>
      <template #header>
        <span>变体详细数据</span>
      </template>
      <el-table :data="results?.variant_results || []"
                border>
        <el-table-column label="变体"
                         min-width="120">
          <template #default="{ row }">
            <div class="variant-cell">
              <el-tag :type="row.is_control ? 'primary' : 'success'">
                {{ row.variant_name }}
              </el-tag>
              <el-tag v-if="row.variant_id === results?.recommended_variant"
                      type="warning"
                      size="small"
                      style="margin-left: 4px;">
                <el-icon>
                  <StarFilled />
                </el-icon>
                推荐
              </el-tag>
              <el-tag v-if="row.is_control"
                      size="small"
                      style="margin-left: 4px;">对照组</el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="traffic_percentage"
                         label="流量分配"
                         width="120">
          <template #default="{ row }">
            {{ row.traffic_percentage }}%
          </template>
        </el-table-column>
        <el-table-column prop="call_count"
                         label="调用次数"
                         width="100"
                         sortable />
        <el-table-column prop="avg_response_time_ms"
                         label="平均响应(ms)"
                         width="140"
                         sortable>
          <template #default="{ row }">
            {{ row.avg_response_time_ms }}
          </template>
        </el-table-column>
        <el-table-column prop="avg_tokens_per_call"
                         label="平均Token"
                         width="120"
                         sortable>
          <template #default="{ row }">
            {{ row.avg_tokens_per_call }}
          </template>
        </el-table-column>
        <el-table-column prop="avg_satisfaction_score"
                         label="平均满意度"
                         width="140"
                         sortable>
          <template #default="{ row }">
            <span v-if="row.avg_satisfaction_score !== null">
              {{ row.avg_satisfaction_score }}
              <el-rate :model-value="row.avg_satisfaction_score"
                       disabled
                       size="small"
                       style="margin-left: 4px; vertical-align: middle;" />
            </span>
            <span v-else
                  class="text-muted">-</span>
          </template>
        </el-table-column>
        <el-table-column label="满意度"
                         width="140"
                         sortable>
          <template #default="{ row }">
            <span class="satisfaction-rate"
                  :class="getSatisfactionClass(row)">
              {{ (row.satisfaction_rate * 100).toFixed(1) }}%
              <span v-if="isSignificantBetter(row)"
                    class="significance-stars">
                {{ getSignificanceStars(row) }}
              </span>
            </span>
          </template>
        </el-table-column>
        <el-table-column prop="total_tokens_consumed"
                         label="总Token消耗"
                         width="140"
                         sortable />
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, nextTick, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { ArrowLeft, StarFilled } from '@element-plus/icons-vue'
import * as echarts from 'echarts'
import { abTestApi } from '@/api'

const route = useRoute()
const router = useRouter()
const results = ref(null)
const loading = ref(false)
const chartRef = ref(null)
const satisfactionChartRef = ref(null)
let chartInstance = null
let satisfactionChartInstance = null

const statusType = computed(() => {
  if (results.value?.status === 'running') return 'success'
  return 'info'
})

const statusLabel = computed(() => {
  if (results.value?.status === 'running') return '进行中'
  return '已完成'
})

const significanceMessage = computed(() => {
  if (!results.value?.statistical_analysis) return ''
  if (results.value.statistical_analysis.is_significant) {
    return '🎉 测试结果具有统计显著性！各变体之间存在真实差异。'
  }
  return '📊 测试结果不具有统计显著性，建议继续收集更多数据或调整变体配置。'
})

const loadResults = async () => {
  loading.value = true
  try {
    const res = await abTestApi.getResults(
      route.params.appId,
      route.params.testId
    )
    results.value = res.data
    await nextTick()
    initCharts()
  } catch (e) {
    ElMessage.error('获取测试结果失败')
  } finally {
    loading.value = false
  }
}

const getSatisfactionClass = (row) => {
  const variants = results.value?.variant_results || []
  const maxRate = Math.max(...variants.map(v => v.satisfaction_rate))
  if (row.satisfaction_rate === maxRate && row.satisfaction_rate > 0) {
    return 'text-success best'
  }
  return ''
}

const isSignificantBetter = (row) => {
  if (!results.value?.statistical_analysis?.is_significant) return false
  const variants = results.value?.variant_results || []
  const maxRate = Math.max(...variants.map(v => v.satisfaction_rate))
  return row.satisfaction_rate === maxRate && row.satisfaction_rate > 0
}

const getSignificanceStars = (row) => {
  const pValue = results.value?.statistical_analysis?.p_value || 1
  if (pValue < 0.001) return '***'
  if (pValue < 0.01) return '**'
  if (pValue < 0.05) return '*'
  return ''
}

const initCharts = () => {
  if (!results.value?.variant_results?.length) return

  const variantNames = results.value.variant_results.map(v => {
    let name = v.variant_name
    if (v.is_control) name += ' (对照)'
    if (v.variant_id === results.value.recommended_variant) name += ' ★'
    return name
  })

  const colors = ['#409eff', '#67c23a', '#e6a23c', '#f56c6c']

  if (chartRef.value) {
    chartInstance = echarts.init(chartRef.value)
    const option = {
      tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'shadow' }
      },
      legend: {
        data: ['调用次数', '平均响应时间(ms)', '平均Token消耗', '平均满意度'],
        top: 0
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        top: '15%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: variantNames,
        axisLabel: { interval: 0, rotate: 0 }
      },
      yAxis: [
        { type: 'value', name: '次数/数值', position: 'left' },
        { type: 'value', name: '评分', position: 'right', min: 0, max: 5 }
      ],
      series: [
        {
          name: '调用次数',
          type: 'bar',
          data: results.value.variant_results.map(v => v.call_count),
          itemStyle: { color: colors[0] }
        },
        {
          name: '平均响应时间(ms)',
          type: 'bar',
          data: results.value.variant_results.map(v => v.avg_response_time_ms),
          itemStyle: { color: colors[1] }
        },
        {
          name: '平均Token消耗',
          type: 'bar',
          data: results.value.variant_results.map(v => v.avg_tokens_per_call),
          itemStyle: { color: colors[2] }
        },
        {
          name: '平均满意度',
          type: 'line',
          yAxisIndex: 1,
          data: results.value.variant_results.map(v => v.avg_satisfaction_score || 0),
          itemStyle: { color: colors[3] },
          lineStyle: { width: 3 },
          symbolSize: 10
        }
      ]
    }
    chartInstance.setOption(option)
  }

  if (satisfactionChartRef.value) {
    satisfactionChartInstance = echarts.init(satisfactionChartRef.value)
    const satisfactionData = results.value.variant_results.map((v, idx) => ({
      value: (v.satisfaction_rate * 100).toFixed(1),
      itemStyle: { color: colors[idx] },
      label: {
        show: true,
        position: 'top',
        formatter: (params) => {
          let suffix = ''
          const pValue = results.value?.statistical_analysis?.p_value || 1
          const maxRate = Math.max(...results.value.variant_results.map(r => r.satisfaction_rate))
          if (v.satisfaction_rate === maxRate && results.value?.statistical_analysis?.is_significant) {
            if (pValue < 0.001) suffix = '***'
            else if (pValue < 0.01) suffix = '**'
            else if (pValue < 0.05) suffix = '*'
          }
          return `${params.value}%${suffix}`
        },
        fontSize: 16,
        fontWeight: 'bold'
      }
    }))

    const option2 = {
      tooltip: {
        trigger: 'item',
        formatter: '{b}: {c}%'
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        top: '10%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: variantNames,
        axisLabel: { interval: 0 }
      },
      yAxis: {
        type: 'value',
        name: '满意度(%)',
        max: 100,
        axisLabel: { formatter: '{value}%' }
      },
      series: [
        {
          name: '满意度',
          type: 'bar',
          data: satisfactionData,
          barWidth: '40%'
        }
      ]
    }
    satisfactionChartInstance.setOption(option2)
  }
}

const goBack = () => {
  router.push(`/apps/${route.params.appId}`)
}

const handleResize = () => {
  chartInstance?.resize()
  satisfactionChartInstance?.resize()
}

onMounted(() => {
  loadResults()
  window.addEventListener('resize', handleResize)
})
</script>

<style scoped>
.ab-test-results {
  padding: 20px;
}
.header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 20px;
}
.header h2 {
  margin: 0;
  flex: 1;
}
.summary-cards {
  margin-bottom: 20px;
}
.stat-item {
  text-align: center;
}
.stat-label {
  font-size: 14px;
  color: #909399;
  margin-bottom: 8px;
}
.stat-value {
  font-size: 28px;
  font-weight: 600;
  color: #303133;
}
.recommended .stat-value {
  color: #409eff;
}
.text-success {
  color: #67c23a;
}
.text-warning {
  color: #e6a23c;
}
.text-primary {
  color: #409eff;
}
.text-muted {
  color: #c0c4cc;
}
.chart-card {
  margin-bottom: 20px;
}
.chart {
  height: 400px;
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.stats-analysis {
  padding: 10px 0;
}
.stats-item {
  text-align: center;
  padding: 20px;
  background: #f5f7fa;
  border-radius: 8px;
}
.stats-label {
  font-size: 14px;
  color: #606266;
  margin-bottom: 8px;
}
.stats-value {
  font-size: 24px;
  font-weight: 600;
}
.variant-cell {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
}
.satisfaction-rate {
  font-weight: 500;
}
.satisfaction-rate.best {
  font-size: 16px;
}
.significance-stars {
  color: #f56c6c;
  font-size: 18px;
  margin-left: 2px;
}
</style>
