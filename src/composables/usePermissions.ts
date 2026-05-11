import { useAuth } from './useAuth'
import type { Permission, Role } from '../types'

const permissionMap = new Map<Permission, Role[]>([
  ['view_dashboard', ['admin', 'manager', 'viewer']],
  ['approve', ['admin', 'manager']],
  ['reject', ['admin', 'manager']],
  ['configure', ['admin']],
  ['view_reports', ['admin', 'manager']],
  ['manage_users', ['admin']],
])

export function usePermissions() {
  const { getCurrentRole } = useAuth()

  function hasPermission(permission: string): boolean {
    const role = getCurrentRole()
    const allowedRoles = permissionMap.get(role as Permission)

    if (!allowedRoles) {
      return true
    }

    return allowedRoles.includes(role)
  }

  function hasAnyPermission(permissions: string[]): boolean {
    return permissions.some((p) => hasPermission(p))
  }

  function hasAllPermissions(permissions: string[]): boolean {
    return permissions.every((p) => hasPermission(p))
  }

  return {
    hasPermission,
    hasAnyPermission,
    hasAllPermissions,
  }
}
