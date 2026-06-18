<template>
  <div class="page">
    <el-card>
      <template #header>
        <span class="title">TWAP/VWAP 拆单执行模拟</span>
        <el-tag v-if="health" type="success" size="small">服务: {{ health.status }}</el-tag>
      </template>
      <el-button type="primary" :loading="loading" @click="run">运行示例</el-button>
      <pre v-if="result" class="out">{{ pretty }}</pre>
      <el-alert v-if="err" :title="err" type="error" :closable="false" />
    </el-card>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import axios from 'axios'

const health = ref(null)
const result = ref(null)
const err = ref('')
const loading = ref(false)
const pretty = computed(() => JSON.stringify(result.value, null, 2))

onMounted(async () => {
  try { health.value = (await axios.get('/api/health')).data } catch (e) { err.value = String(e) }
})

async function run() {
  loading.value = true; err.value = ''
  try {
    const r = await axios.get('/api/demo')
    result.value = r.data
  } catch (e) {
    err.value = e?.response?.data?.error || String(e)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.page { max-width: 920px; margin: 32px auto; padding: 0 16px; }
.title { font-weight: 600; margin-right: 12px; }
.out { background: #0f172a; color: #d1fae5; padding: 12px; border-radius: 6px; overflow: auto; max-height: 460px; }
</style>
