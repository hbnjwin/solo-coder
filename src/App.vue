<template>
  <div class="app">
    <NavBar
      :current-page="activePage"
      :current-role="currentRole"
      @navigate="activePage = $event"
      @role-change="handleRoleChange"
    />
    <main class="main-content">
      <PageDashboard v-if="activePage === 'dashboard'" />
      <PageApproval v-if="activePage === 'approval'" />
      <PageSettings v-if="activePage === 'settings'" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from './composables/useAuth'
import type { Role } from './types'
import NavBar from './components/NavBar.vue'
import PageDashboard from './components/PageDashboard.vue'
import PageApproval from './components/PageApproval.vue'
import PageSettings from './components/PageSettings.vue'

const { user, switchRole } = useAuth()
const activePage = ref('dashboard')
const currentRole = ref<string>(user.value.role)

function handleRoleChange(role: string) {
  currentRole.value = role
  switchRole(role as Role)
}
</script>

<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
.app { min-height: 100vh; background: #f5f5f5; }
.main-content { max-width: 960px; margin: 24px auto; background: #fff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.06); }
</style>
