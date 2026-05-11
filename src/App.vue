<template>
  <div class="container">
    <h2>审批系统</h2>
    <!-- BUG: no permission control -->
    <nav>
      <a href="#" @click.prevent="page='list'">列表</a>
      <a href="#" @click.prevent="page='submit'">提交</a>
      <a href="#" @click.prevent="page='approve'">审批</a>
      <a href="#" @click.prevent="page='report'">报表</a>
      <a href="#" @click.prevent="page='config'">配置</a>
    </nav>
    <div class="role-select">当前角色: <select v-model="currentRole">
      <option value="employee">普通员工</option><option value="manager">部门经理</option>
      <option value="gm">总经理</option><option value="admin">系统管理员</option>
    </select></div>
    <div class="content"><p>当前页面: {{ page }}</p>
      <button v-if="page==='list'">删除</button>
      <button v-if="page==='list'">审批</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const currentRole = ref('employee')
const page = ref('list')
</script>
<style>
.container { padding: 20px; }
nav { margin: 16px 0; display: flex; gap: 12px; }
nav a { padding: 6px 12px; background: #1890ff; color: white; text-decoration: none; border-radius: 4px; }
.role-select { margin: 12px 0; }
.content { padding: 16px; border: 1px solid #ddd; border-radius: 4px; }
button { margin: 4px; padding: 6px 16px; }
</style>