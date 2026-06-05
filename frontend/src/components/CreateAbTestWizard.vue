<template>
  <el-dialog v-model="visible"
             title="创建A/B测试"
             width="800px"
             :close-on-click-modal="false"
             @closed="handleClosed">
    <el-steps :active="activeStep"
              finish-status="success"
              align-center>
      <el-step title="基本信息" />
      <el-step title="配置变体" />
      <el-step title="流量分配" />
    </el-steps>

    <div class="wizard-content">
      <div v-if="activeStep === 0"
           class="step-content">
        <el-form :model="form"
                 label-width="100px">
          <el-form-item label="测试名称"
                        required>
            <el-input v-model="form.test_name"
                      placeholder="请输入测试名称" />
          </el-form-item>
          <el-form-item label="描述">
            <el-input v-model="form.description"
                      type="textarea"
                      :rows="2"
                      placeholder="请输入测试描述" />
          </el-form-item>
          <el-form-item label="测试目标"
                        required>
            <el-radio-group v-model="form.target_type">
              <el-radio value="prompt">提示词</el-radio>
              <el-radio value="model">模型</el-radio>
              <el-radio value="params">参数</el-radio>
              <el-radio value="all">全部</el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="变体数量"
                        required>
            <el-radio-group v-model="variantCount"
                            @change="handleVariantCountChange">
              <el-radio :value="2">2 个</el-radio>
              <el-radio :value="3">3 个</el-radio>
              <el-radio :value="4">4 个</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-form>
      </div>

      <div v-if="activeStep === 1"
           class="step-content">
        <div v-for="(variant, idx) in form.variants"
             :key="idx"
             class="variant-card">
          <div class="variant-header">
            <el-tag :type="idx === 0 ? 'primary' : 'success'"
                    size="large">
              {{ variant.variant_name }}
              <span v-if="idx === 0"
                    style="margin-left: 4px;">(对照组)</span>
            </el-tag>
          </div>
          <el-form :model="variant"
                   label-width="100px">
            <el-form-item label="变体名称">
              <el-input v-model="variant.variant_name" />
            </el-form-item>
            <el-form-item v-if="showField('prompt')"
                          label="系统提示词">
              <el-input v-model="variant.system_prompt"
                        type="textarea"
                        :rows="4" />
            </el-form-item>
            <el-form-item v-if="showField('model')"
                          label="模型">
              <el-select v-model="variant.model_id"
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
            <el-form-item v-if="showField('params')"
                          label="温度">
              <el-slider v-model="variant.temperature"
                         :min="0"
                         :max="1"
                         :step="0.1"
                         show-input />
            </el-form-item>
            <el-form-item v-if="showField('params')"
                          label="最大Token">
              <el-input-number v-model="variant.max_tokens"
                               :min="100"
                               :max="4000"
                               :step="100"
                               style="width: 100%" />
            </el-form-item>
          </el-form>
        </div>
      </div>

      <div v-if="activeStep === 2"
           class="step-content">
        <el-form :model="form"
                 label-width="100px">
          <el-form-item label="分配方式">
            <el-radio-group v-model="trafficMode"
                            @change="handleTrafficModeChange">
              <el-radio value="even">均匀分配</el-radio>
              <el-radio value="custom">自定义</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-form>

        <div class="traffic-config">
          <div v-for="(variant, idx) in form.variants"
               :key="idx"
               class="traffic-item">
            <div class="traffic-label">
              <el-tag :type="idx === 0 ? 'primary' : 'success'">
                {{ variant.variant_name }}
              </el-tag>
              <span v-if="idx === 0"
                    class="control-tag">对照组</span>
            </div>
            <div class="traffic-slider">
              <el-slider v-model="variant.traffic_percentage"
                         :min="0"
                         :max="100"
                         :step="1"
                         :disabled="trafficMode === 'even'"
                         show-input
                         input-size="small" />
            </div>
          </div>
          <div class="traffic-total">
            <span>总流量：</span>
            <span :class="{ 'text-danger': totalTraffic !== 100 }">{{ totalTraffic }}%</span>
            <el-tag v-if="totalTraffic !== 100"
                    type="danger"
                    size="small">流量分配必须等于100%</el-tag>
          </div>
        </div>

        <el-alert title="配置预览"
                  type="info"
                  :closable="false"
                  style="margin-top: 20px;">
          <div class="preview">
            <p><strong>测试名称：</strong>{{ form.test_name }}</p>
            <p><strong>测试目标：</strong>{{ getTargetTypeLabel(form.target_type) }}</p>
            <p><strong>变体数量：</strong>{{ form.variants.length }} 个</p>
            <p v-for="(v, idx) in form.variants"
               :key="idx">
              <strong>{{ v.variant_name }}:</strong> {{ v.traffic_percentage }}% 流量
            </p>
          </div>
        </el-alert>
      </div>
    </div>

    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button v-if="activeStep > 0"
                 @click="prevStep">上一步</el-button>
      <el-button v-if="activeStep < 2"
                 type="primary"
                 @click="nextStep"
                 :disabled="!canProceed">下一步</el-button>
      <el-button v-if="activeStep === 2"
                 type="primary"
                 @click="handleCreate"
                 :loading="creating"
                 :disabled="totalTraffic !== 100">
        创建测试
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { abTestApi, appApi } from '@/api'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  appId: {
    type: Number,
    required: true
  }
})

const emit = defineEmits(['update:modelValue', 'created'])

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const activeStep = ref(0)
const variantCount = ref(2)
const trafficMode = ref('even')
const creating = ref(false)
const appConfig = ref(null)

const form = ref({
  test_name: '',
  description: '',
  target_type: 'all',
  variants: []
})

const totalTraffic = computed(() => {
  return form.value.variants.reduce((sum, v) => sum + (v.traffic_percentage || 0), 0)
})

const canProceed = computed(() => {
  if (activeStep.value === 0) {
    return form.value.test_name && form.value.target_type && variantCount.value >= 2
  }
  if (activeStep.value === 1) {
    return form.value.variants.every(v => v.variant_name)
  }
  return true
})

const loadAppConfig = async () => {
  try {
    const res = await appApi.getApp(props.appId)
    appConfig.value = res.data
  } catch (e) {
    console.error('Failed to load app config', e)
  }
}

const initVariants = (count) => {
  const defaultPercentage = trafficMode.value === 'even' ? 100 / count : 0
  const names = ['Variant A (对照组)', 'Variant B', 'Variant C', 'Variant D']
  form.value.variants = Array.from({ length: count }, (_, idx) => ({
    variant_name: names[idx] || `Variant ${String.fromCharCode(65 + idx)}`,
    is_control: idx === 0,
    traffic_percentage: defaultPercentage,
    system_prompt: appConfig.value?.system_prompt || '',
    model_id: appConfig.value?.model_id || 'gpt-3.5-turbo',
    temperature: appConfig.value?.temperature ?? 0.7,
    max_tokens: appConfig.value?.max_tokens ?? 1000
  }))
}

const showField = (field) => {
  const type = form.value.target_type
  if (type === 'all') return true
  return type === field
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

const handleVariantCountChange = () => {
  initVariants(variantCount.value)
}

const handleTrafficModeChange = () => {
  if (trafficMode.value === 'even') {
    const percentage = 100 / form.value.variants.length
    form.value.variants.forEach(v => {
      v.traffic_percentage = percentage
    })
  }
}

const nextStep = () => {
  if (activeStep.value < 2) {
    activeStep.value++
  }
}

const prevStep = () => {
  if (activeStep.value > 0) {
    activeStep.value--
  }
}

const handleCreate = async () => {
  if (totalTraffic.value !== 100) {
    ElMessage.warning('流量分配总和必须等于100%')
    return
  }
  creating.value = true
  try {
    await abTestApi.createTest(props.appId, form.value)
    ElMessage.success('A/B测试创建成功')
    emit('created')
    visible.value = false
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '创建失败')
  } finally {
    creating.value = false
  }
}

const handleClosed = () => {
  activeStep.value = 0
  variantCount.value = 2
  trafficMode.value = 'even'
  form.value = {
    test_name: '',
    description: '',
    target_type: 'all',
    variants: []
  }
}

watch(() => props.modelValue, (val) => {
  if (val) {
    loadAppConfig().then(() => {
      initVariants(variantCount.value)
    })
  }
})
</script>

<style scoped>
.wizard-content {
  padding: 30px 0;
  min-height: 400px;
}
.step-content {
  padding: 0 20px;
}
.variant-card {
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  background: #fafafa;
}
.variant-header {
  margin-bottom: 16px;
}
.traffic-config {
  margin-top: 20px;
}
.traffic-item {
  margin-bottom: 20px;
}
.traffic-label {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}
.control-tag {
  font-size: 12px;
  color: #909399;
}
.traffic-total {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 4px;
  font-size: 14px;
}
.text-danger {
  color: #f56c6c;
  font-weight: 500;
}
.preview p {
  margin: 6px 0;
}
</style>
