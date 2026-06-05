import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000
})

export const appApi = {
  getApps: () => api.get('/apps'),
  getApp: (id) => api.get(`/apps/${id}`),
  createApp: (data) => api.post('/apps', data),
  updateApp: (id, data) => api.put(`/apps/${id}`, data),
  deleteApp: (id) => api.delete(`/apps/${id}`),
  callApp: (id, query) => api.post(`/apps/${id}/call`, { query }),
  getCallLogs: (id) => api.get(`/apps/${id}/call-logs`)
}

export const abTestApi = {
  getTests: (appId) => api.get(`/apps/${appId}/ab-tests`),
  getTest: (appId, testId) => api.get(`/apps/${appId}/ab-tests/${testId}`),
  createTest: (appId, data) => api.post(`/apps/${appId}/ab-tests`, data),
  stopTest: (appId, testId) => api.post(`/apps/${appId}/ab-tests/${testId}/stop`),
  getResults: (appId, testId) => api.get(`/apps/${appId}/ab-tests/${testId}/results`)
}

export const feedbackApi = {
  submit: (logId, data) => api.post(`/call-logs/${logId}/feedback`, data)
}

export default api
