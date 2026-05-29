<template>
  <el-config-provider>
    <router-view />
  </el-config-provider>
</template>

<script setup>
import { ElConfigProvider } from 'element-plus'
</script>

<style>
body {
  margin: 0;
  padding: 0;
}
#app {
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', Arial, sans-serif;
}
</style>
