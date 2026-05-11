import { ref, readonly } from 'vue'
import type { User, Role } from '../types'

const currentUser = ref<User>({
  id: 'u-001',
  name: '张三',
  role: 'viewer',
})

let refreshTimer: ReturnType<typeof setTimeout> | null = null

function refreshToken(): Promise<void> {
  return new Promise((resolve) => {
    if (refreshTimer) {
      clearTimeout(refreshTimer)
    }
    refreshTimer = setTimeout(() => {
      refreshTimer = null
      resolve()
    }, 300)
  })
}

export function useAuth() {
  function switchRole(role: Role) {
    currentUser.value = {
      ...currentUser.value,
      role,
    }
    refreshToken()
  }

  function getCurrentRole(): Role {
    return currentUser.value.role
  }

  return {
    user: readonly(currentUser),
    switchRole,
    getCurrentRole,
    refreshToken,
  }
}
