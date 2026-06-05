<template>
  <div class="app-detail"
       v-loading="loading">
    <div class="header">
      <el-button @click="goBack">
        <el-icon>
          <ArrowLeft />
        </el-icon>
        返回
      </el-button>
      <h2>{{ app?.name }}</h2>
    </div>

    <el-tabs v-model="activeTab">
      <el-tab-pane label="应用配置"
                   name="config">
        <AppConfig :app="app"
                   @updated="handleAppUpdated" />
      </el-tab-pane>
      <el-tab-pane label="应用调用"
                   name="call">
        <AppCall :app="app" />
      </el-tab-pane>
      <el-tab-pane label="A/B测试"
                   name="abtest">
        <AbTestList :appId="Number(route.params.id)" />
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { ArrowLeft } from '@element-plus/icons-vue'
import { appApi } from '@/api'
import AppConfig from '@/components/AppConfig.vue'
import AppCall from '@/components/AppCall.vue'
import AbTestList from '@/components/AbTestList.vue'

const route = useRoute()
const router = useRouter()
const app = ref(null)
const loading = ref(false)
const activeTab = ref('config')

const fetchApp = async () => {
  loading.value = true
  try {
    const res = await appApi.getApp(route.params.id)
    app.value = res.data
  } catch (e) {
    ElMessage.error('获取应用信息失败')
  } finally {
    loading.value = false
  }
}

const handleAppUpdated = () => {
  fetchApp()
}

const goBack = () => {
  router.push('/')
}

onMounted(() => {
  fetchApp()
})
</script>

<style scoped>
.app-detail {
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
}
</style>
