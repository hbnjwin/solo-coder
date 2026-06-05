﻿import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/AppList.vue')
  },
  {
    path: '/apps/:id',
    name: 'AppDetail',
    component: () => import('@/views/AppDetail.vue')
  },
  {
    path: '/apps/:appId/ab-tests/:testId/results',
    name: 'AbTestResults',
    component: () => import('@/views/AbTestResults.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
