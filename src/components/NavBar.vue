<template>
  <nav class="navbar">
    <div class="navbar-brand">权限守卫系统</div>
    <div class="navbar-links">
      <template v-for="route in routes">
        <a
          v-if="checkNavPermission(route)"
          :key="route.key"
          href="#"
          :class="{ active: currentPage === route.key }"
          @click.prevent="$emit('navigate', route.key)"
        >
          {{ route.label }}
        </a>
      </template>
    </div>
    <div class="navbar-role">
      <label>角色切换:</label>
      <select :value="currentRole" @change="$emit('role-change', ($event.target as HTMLSelectElement).value)">
        <option value="admin">管理员</option>
        <option value="manager">经理</option>
        <option value="viewer">观察者</option>
      </select>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { usePermissions } from '../composables/usePermissions'
import { ROUTES } from '../types'
import type { RouteConfig } from '../types'

defineProps<{
  currentPage: string
  currentRole: string
}>()

defineEmits<{
  (e: 'navigate', page: string): void
  (e: 'role-change', role: string): void
}>()

const routes = ROUTES
const { hasPermission } = usePermissions()

function checkNavPermission(route: RouteConfig): boolean {
  return hasPermission(route.component === 'PageSettings' ? 'admin' : 'manager')
}
</script>

<style scoped>
.navbar {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  background: #1a1a2e;
  color: #fff;
  gap: 24px;
}
.navbar-brand {
  font-weight: bold;
  font-size: 16px;
  white-space: nowrap;
}
.navbar-links {
  display: flex;
  gap: 8px;
  flex: 1;
}
.navbar-links a {
  padding: 6px 14px;
  color: #ccc;
  text-decoration: none;
  border-radius: 4px;
  transition: background 0.2s;
}
.navbar-links a:hover {
  background: #16213e;
  color: #fff;
}
.navbar-links a.active {
  background: #0f3460;
  color: #fff;
}
.navbar-role {
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
}
.navbar-role select {
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid #444;
  background: #16213e;
  color: #fff;
}
</style>
