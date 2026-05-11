export type Role = 'admin' | 'manager' | 'viewer'

export type Permission =
  | 'view_dashboard'
  | 'approve'
  | 'reject'
  | 'configure'
  | 'view_reports'
  | 'manage_users'

export interface User {
  id: string
  name: string
  role: Role
}

export interface RouteConfig {
  key: string
  label: string
  permission: Permission
  component: string
}

export const ROUTES: RouteConfig[] = [
  { key: 'dashboard', label: '仪表盘', permission: 'view_dashboard', component: 'PageDashboard' },
  { key: 'approval', label: '审批管理', permission: 'approve', component: 'PageApproval' },
  { key: 'settings', label: '系统设置', permission: 'configure', component: 'PageSettings' },
]
