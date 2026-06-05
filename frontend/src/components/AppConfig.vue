<template>
  <div class="app-config">
    <el-form :model="form"
             label-width="120px"
             v-loading="saving">
      <el-form-item label="应用名称">
        <el-input v-model="form.name" />
      </el-form-item>
      <el-form-item label="描述">
        <el-input v-model="form.description"
                  type="textarea"
                  :rows="2" />
      </el-form-item>
      <el-form-item label="系统提示词">
        <el-input v-model="form.system_prompt"
                  type="textarea"
                  :rows="6" />
      </el-form-item>
      <el-form-item label="模型">
        <el-select v-model="form.model_id"
                   style="width: 100%">
          <el-option label="GPT-3.5 Turbo"
                     value="gpt-3.5-turbo" />
          <el-option label="GPT-4"
                     value="gpt-4" />
          <el-option label="GPT-4o"
                     value="gpt-4o" />
          <el-option label="Claude 3 Opus"
                     value="claude-3-opus" />
        </el-select>
      </el-form-item>
      <el-form-item label="温度">
        <el-slider v-model="form.temperature"
                   :min="0"
                   :max="1"
                   :step="0.1"
                   show-input />
      </el-form-item>
      <el-form-item label="最大Token">
        <el-input-number v-model="form.max_tokens"
                         :min="100"
                         :max="4000"
                         :step="100"
                         style="width: 100%" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary"
                   @click="handleSave"
                   :loading="saving">保存配置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { appApi } from '@/api'

const props = defineProps({
  app: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['updated'])

const form = ref({
  name: '',
  description: '',
  system_prompt: '',
  model_id: 'gpt-3.5-turbo',
  temperature: 0.7,
  max_tokens: 1000
})
const saving = ref(false)

watch(() => props.app, (newApp) => {
  if (newApp) {
    form.value = {
      name: newApp.name || '',
      description: newApp.description || '',
      system_prompt: newApp.system_prompt || '',
      model_id: newApp.model_id || 'gpt-3.5-turbo',
      temperature: newApp.temperature ?? 0.7,
      max_tokens: newApp.max_tokens ?? 1000
    }
  }
}, { immediate: true, deep: true })

const handleSave = async () => {
  if (!form.value.name) {
    ElMessage.warning('请输入应用名称')
    return
  }
  saving.value = true
  try {
    await appApi.updateApp(props.app.id, form.value)
    ElMessage.success('保存成功')
    emit('updated')
  } catch (e) {
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.app-config {
  max-width: 800px;
}
</style>
