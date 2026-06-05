<template>
  <div class="app-call">
    <div class="chat-container">
      <div class="chat-messages">
        <div v-if="messages.length === 0"
             class="empty-state">
          <el-empty description="请输入问题开始对话" />
        </div>
        <div v-for="(msg, idx) in messages"
             :key="idx"
             class="message-item"
             :class="msg.role">
          <div class="message-avatar">
            <el-icon v-if="msg.role === 'user'">
              <User />
            </el-icon>
            <el-icon v-else>
              <Cpu />
            </el-icon>
          </div>
          <div class="message-content">
            <div class="message-text">{{ msg.content }}</div>
            <div v-if="msg.variant_name"
                 class="message-meta">
              <el-tag size="small"
                      type="info">变体: {{ msg.variant_name }}</el-tag>
              <el-tag size="small">模型: {{ msg.model_id }}</el-tag>
              <el-tag size="small">耗时: {{ msg.response_time_ms }}ms</el-tag>
              <el-tag size="small">Token: {{ msg.tokens_consumed }}</el-tag>
            </div>
            <div v-if="msg.role === 'assistant' && !msg.feedbackSubmitted"
                 class="feedback">
              <span class="feedback-label">满意度：</span>
              <el-rate v-model="msg.rating"
                       :max="5"
                       size="small"
                       @change="(val) => handleRate(msg, val)" />
            </div>
            <div v-if="msg.feedbackSubmitted"
                 class="feedback-submitted">
              <el-tag size="small"
                      type="success">已评价：{{ msg.rating }} 星</el-tag>
            </div>
          </div>
        </div>
      </div>

      <div class="chat-input">
        <el-input v-model="inputText"
                  placeholder="请输入您的问题..."
                  type="textarea"
                  :rows="2"
                  @keydown.enter.ctrl="handleSend" />
        <el-button type="primary"
                   @click="handleSend"
                   :loading="calling">
          发送 (Ctrl+Enter)
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { User, Cpu } from '@element-plus/icons-vue'
import { appApi, feedbackApi } from '@/api'

const props = defineProps({
  app: {
    type: Object,
    required: true
  }
})

const messages = ref([])
const inputText = ref('')
const calling = ref(false)

const handleSend = async () => {
  if (!inputText.value.trim()) {
    ElMessage.warning('请输入问题')
    return
  }
  const query = inputText.value.trim()
  inputText.value = ''

  messages.value.push({
    role: 'user',
    content: query
  })

  calling.value = true
  try {
    const res = await appApi.callApp(props.app.id, query)
    const data = res.data
    messages.value.push({
      role: 'assistant',
      content: data.response,
      call_log_id: data.call_log_id,
      variant_id: data.variant_id,
      variant_name: data.variant_name,
      model_id: data.model_id,
      response_time_ms: data.response_time_ms,
      tokens_consumed: data.tokens_consumed,
      rating: 0,
      feedbackSubmitted: false
    })
  } catch (e) {
    ElMessage.error('调用失败')
  } finally {
    calling.value = false
  }
}

const handleRate = async (msg, val) => {
  try {
    await feedbackApi.submit(msg.call_log_id, {
      satisfaction_score: val
    })
    msg.feedbackSubmitted = true
    ElMessage.success('感谢您的评价')
  } catch (e) {
    ElMessage.error('评价失败')
  }
}
</script>

<style scoped>
.app-call {
  max-width: 900px;
}
.chat-container {
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 600px;
}
.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  background: #f5f7fa;
}
.empty-state {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.message-item {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
}
.message-item.user {
  flex-direction: row-reverse;
}
.message-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: #409eff;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.message-item.user .message-avatar {
  background: #67c23a;
}
.message-content {
  max-width: 70%;
  background: white;
  padding: 12px 16px;
  border-radius: 8px;
  box-shadow: 0 1px 2px rgba(0,0,0,0.05);
}
.message-item.user .message-content {
  background: #ecf5ff;
}
.message-text {
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}
.message-meta {
  margin-top: 8px;
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}
.feedback {
  margin-top: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
}
.feedback-label {
  font-size: 13px;
  color: #606266;
}
.feedback-submitted {
  margin-top: 8px;
}
.chat-input {
  padding: 16px;
  background: white;
  border-top: 1px solid #e4e7ed;
  display: flex;
  gap: 12px;
  align-items: flex-end;
}
.chat-input .el-input {
  flex: 1;
}
</style>
