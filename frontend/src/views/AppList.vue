<template>
  <div class="app-list">
    <div class="header">
      <h2>AI 知识库应用列表</h2>
      <el-button type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>
        新建应用
      </el-button>
    </div>

    <el-table :data="apps" v-loading="loading" style="width: 100%" @row-click="goToDetail">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="应用名称" min-width="150">
        <template #default="{ row }">
          <span class="app-name">{{ row.name }}</span>
        </template>
      </el-table-column>
      <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
      <el-table-column prop="model_id" label="模型" width="150" />
      <el-table-column prop="temperature" label="温度" width="100" />
      <el-table-column prop="created_at" label="创建时间" width="180">
        <template #default="{ row }">
          {{ formatDate(row.created_at) }}
        </template>
      </el-table-column>
      <el-table-column label="操作" width="200" fixed="right">
        <template #default="{ row }">
          <el-button type="primary" link @click.stop="goToDetail(row)">详情</el-button>
          <el-button type="danger" link @click.stop="handleDelete(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showCreateDialog" title="新建应用" width="600px">
      <el-form :model="form" label-width="100px">
        <el-form-item label="应用名称">
          <el-input v-model="form.name" placeholder="请输入应用名称" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="form.description" type="textarea" :rows="2" placeholder="请输入应用描述" />
        </el-form-item>
        <el-form-item label="系统提示词">
          <el-input v-model="form.system_prompt" type="textarea" :rows="4" placeholder="请输入系统提示词" />
        </el-form-item>
        <el-form-item label="模型">
          <el-select v-model="form.model_id" style="width: 100%">
            <el-option label="GPT-3.5 Turbo" value="gpt-3.5-turbo" />
            <el-option label="GPT-4" value="gpt-4" />
            <el-option label="GPT-4o" value="gpt-4o" />
            <el-option label="Claude 3 Opus" value="claude-3-opus" />
          </el-select>
        </el-form-item>
        <el-form-item label="温度">
          <el-slider v-model="form.temperature" :min="0" :max="1" :step="0.1" show-input />
        </el-form-item>
        <el-form-item label="最大Token">
          <el-input-number v-model="form.max_tokens" :min="100" :max="4000" :step="100" style="width: 100%" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDialog = false">取消</el-button>
        <el-button type="primary" @click="handleCreate" :loading="creating">创建</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { appApi } from '@/api'

const router = useRouter()
const apps = ref([])
const loading = ref(false)
const creating = ref(false)
const showCreateDialog = ref(false)
const form = ref({
  name: '',
  description: '',
  system_prompt: '',
  model_id: 'gpt-3.5-turbo',
  temperature: 0.7,
  max_tokens: 1000
})

const fetchApps = async () => {
  loading.value = true
  try {
    const res = await appApi.getApps()
    apps.value = res.data
  } catch (e) {
    ElMessage.error('获取应用列表失败')
  } finally {
    loading.value = false
  }
}

const goToDetail = (row) => {
  router.push(`/apps/${row.id}`)
}

const handleCreate = async () => {
  if (!form.value.name) {
    ElMessage.warning('请输入应用名称')
    return
  }
  creating.value = true
  try {
    await appApi.createApp(form.value)
    ElMessage.success('创建成功')
    showCreateDialog.value = false
    form.value = {
      name: '',
      description: '',
      system_prompt: '',
      model_id: 'gpt-3.5-turbo',
      temperature: 0.7,
      max_tokens: 1000
    }
    fetchApps()
  } catch (e) {
    ElMessage.error('创建失败')
  } finally {
    creating.value = false
  }
}

const handleDelete = async (row) => {
  try {
    await ElMessageBox.confirm(`确定要删除应用「${row.name}」吗？`, '提示', {
      type: 'warning'
    })
    await appApi.deleteApp(row.id)
    ElMessage.success('删除成功')
    fetchApps()
  } catch (e) {
    if (e !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const formatDate = (date) => {
  if (!date) return ''
  return new Date(date).toLocaleString('zh-CN')
}

onMounted(() => {
  fetchApps()
})
</script>

<style scoped>
.app-list {
  padding: 20px;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
.app-name {
  font-weight: 500;
  color: #409eff;
  cursor: pointer;
}
</style>
