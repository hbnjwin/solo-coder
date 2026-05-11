export type NotificationPriority = 'high' | 'medium' | 'low'

export interface Notification {
  id: string
  title: string
  body: string
  priority: NotificationPriority
  read: boolean
  timestamp: number
}

export interface NotificationState {
  notifications: Notification[]
  isPanelOpen: boolean
}
