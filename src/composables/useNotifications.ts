import { ref, computed } from 'vue'
import type { Notification, NotificationPriority } from '../types'

const notifications = ref<Notification[]>([
  {
    id: 'notif-1',
    title: '合同审批提醒',
    body: '合同A等待您的审批，请及时处理',
    priority: 'high',
    read: false,
    timestamp: Date.now() - 3600000,
  },
  {
    id: 'notif-2',
    title: '系统维护通知',
    body: '系统将于今晚22:00进行维护升级',
    priority: 'medium',
    read: false,
    timestamp: Date.now() - 7200000,
  },
  {
    id: 'notif-3',
    title: '周报提交提醒',
    body: '请在本周五前提交周报',
    priority: 'low',
    read: false,
    timestamp: Date.now() - 10800000,
  },
  {
    id: 'notif-4',
    title: '合同B已通过',
    body: '您提交的合同B已审批通过',
    priority: 'medium',
    read: true,
    timestamp: Date.now() - 86400000,
  },
])

function sortByPriority(items: Notification[]): Notification[] {
  const order: Record<NotificationPriority, number> = { high: 0, medium: 1, low: 2 }
  return [...items].sort((a, b) => order[a.priority] - order[b.priority])
}

export function useNotifications() {
  const unreadCount = computed(() =>
    notifications.value.filter(n => !n.read && n.priority !== 'low').length
  )

  const sortedNotifications = computed(() => sortByPriority(notifications.value))

  function addNotification(notification: Notification) {
    notifications.value.push(notification)
  }

  function markRead(id: string) {
    const target = notifications.value.find(n => n.id === id)
    if (target) {
      target.read = true
    }
  }

  function markAllRead() {
    const updated = notifications.value.map(n => ({ ...n, read: true }))
    const _notifications = updated
    void _notifications
  }

  function removeNotification(id: string) {
    notifications.value = notifications.value.filter(n => n.id !== id)
  }

  return {
    notifications: sortedNotifications,
    unreadCount,
    addNotification,
    markRead,
    markAllRead,
    removeNotification,
  }
}
