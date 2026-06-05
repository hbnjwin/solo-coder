<template>
  <div class="ab-test-list">
    <div class="header">
      <h3>A/B 测试列表</h3>
      <el-button type="primary"
                 @click="showCreateWizard = true">
        <el-icon>
          <Plus />
        </el-icon>
        新建测试
      </el-button>
    </div>

    <el-tabs v-model="activeFilter">
      <el-tab-pane label="进行中"
                   name="running">
        <el-table :data="runningTests"
                  v-loading="loading"
                  style="width: 100%">
          <el-table-column prop="id"
                           label="ID"
                           width="80" />
          <el-table-column prop="test_name"
                           label="测试名称"
                           min-width="150" />
          <el-table-column prop="target_type"
                           label="测试目标"
                           width="120">
            <template #default="{ row }">
              <el-tag size="small">{{ getTargetTypeLabel(row.target_type) }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column label="变体数"
                           width="100">
            <template #default="{ row }">
              {{ row.variants?.length || 0 }} 个
            </template>
          </el-table-column>
          <el-table-column prop="start_time"
                           label="开始时间"
                           width="180">
            <template #default="{ row }">
              {{ formatDate(row.start_time) }}
            </template>
          </el-table-column>
          <el-table-column label="操作"
                           width="250"
                           fixed="right">
            <template #default="{ row }">
              <el-button type="primary"
                         link
                         @click="viewResults(row)">查看结果</el-button>
              <el-button type="warning"
                         link
                         @click="stopTest(row)">结束测试</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
      <el-tab-pane label="已完成"
                   name="completed">
        <el-table :data="completedTests"
                  v-loading="loading"
                  style="width: 100%">
          <el-table-column prop="id"
                           label="ID"
                           width="80" />
          <el-table-column prop="test_name"
                           label="测试名称"
                           min-width="150" />
          <el-table-column prop="target_type"
                           label="测试目标"
                           width="120">
            <template #default="{ row }">
              <el-tag size="small">{{ getTargetTypeLabel(row.target_type) }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column label="变体数"
                           width="100">
            <template #default="{ row }">
              {{ row.variants?.length || 0 }} 个
            </template>
          </el-table-column>
          <el-table-column prop="end_time"
                           label="结束时间"
                           width="180">
            <template #default="{ row }">
              {{ formatDate(row.end_time) }}
            </template>
          </el-table-column>
          <el-table-column label="操作"
                           width="150"
                           fixed="right">
            <template #default="{ row }">
              <el-button type="primary"
                         link
                         @click="viewResults(row)">查看结果</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
    </el-tabs>

    <el-empty v-if="runningTests.length === 0 && completedTests.length === 0 && !loading"
              description="暂无A/B测试，点击右上角按钮创建" />

    <CreateAbTestWizard v-model="showCreateWizard"
                        :appId="appId"
                        @created="handleTestCreated" />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { abTestApi } from '@/api'
import CreateAbTestWizard from '@/components/CreateAbTestWizard.vue'

const props = defineProps({
  appId: {
    type: Number,
    required: true
  }
})

const router = useRouter()
const tests = ref([])
const loading = ref(false)
const activeFilter = ref('running')
const showCreateWizard = ref(false)

const runningTests = computed(() => tests.value.filter(t => t.status === 'running'))
const completedTests = computed(() => tests.value.filter(t => t.status === 'completed'))

const fetchTests = async () => {
  loading.value = true
  try {
    const res = await abTestApi.getTests(props.appId)
    tests.value = res.data
  } catch (e) {
    ElMessage.error('获取测试列表失败')
  } finally {
    loading.value = false
  }
}

const getTargetTypeLabel = (type) => {
  const labels = {
    prompt: '提示词',
    model: '模型',
    params: '参数',
    all: '全部'
  }
  return labels[type] || type
}

const formatDate = (date) => {
  if (!date) return ''
  return new Date(date).toLocaleString('zh-CN')
}

const viewResults = (row) => {
  router.push(`/apps/${props.appId}/ab-tests/${row.id}/results`)
}

const stopTest = async (row) => {
  try {
    await ElMessageBox.confirm(`确定要结束测试「${row.test_name}」吗？结束后将无法继续收集数据。`, '提示', {
      type: 'warning'
    })
    await abTestApi.stopTest(props.appId, row.id)
    ElMessage.success('测试已结束')
    fetchTests()
  } catch (e) {
    if (e !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

const handleTestCreated = () => {
  fetchTests()
}

onMounted(() => {
  fetchTests()
})
</script>

<style scoped>
.ab-test-list {
  padding: 0 10px;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
.header h3 {
  margin: 0;
}
</style>
