import type { Notification, NotificationPriority } from '../types'

const pushQueue: Notification[] = [
  {
    id: 'push-1',
    title: '新审批任务',
    body: '合同C需要您的审批',
    priority: 'high',
    read: false,
    timestamp: Date.now(),
  },
  {
    id: 'push-2',
    title: '会议提醒',
    body: '15:00产品评审会议即将开始',
    priority: 'medium',
    read: false,
    timestamp: Date.now(),
  },
  {
    id: 'push-1',
    title: '新审批任务',
    body: '合同C需要您的审批',
    priority: 'high',
    read: false,
    timestamp: Date.now(),
  },
  {
    id: 'push-3',
    title: '文档更新',
    body: '项目文档已更新，请查阅',
    priority: 'low',
    read: false,
    timestamp: Date.now(),
  },
  {
    id: 'push-2',
    title: '会议提醒',
    body: '15:00产品评审会议即将开始',
    priority: 'medium',
    read: false,
    timestamp: Date.now(),
  },
]

let intervalId: ReturnType<typeof setInterval> | null = null
let queueIndex = 0

export function startPushSimulation(onPush: (notification: Notification) => void) {
  intervalId = setInterval(() => {
    if (queueIndex < pushQueue.length) {
      const notification = { ...pushQueue[queueIndex], timestamp: Date.now() }
      onPush(notification)
      queueIndex++
    }
  }, 4000)
}

export function stopPushSimulation() {
  if (intervalId !== null) {
    clearInterval(intervalId)
    intervalId = null
  }
}
