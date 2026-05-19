import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    redirect: '/home'
  }
]

export default createRouter({
  history: createWebHashHistory(),
  routes
})
