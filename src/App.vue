<template>
  <div class="editor-layout">
    <NodePanel />
    <DesignerCanvas />
    <PropertyPanel />
  </div>
</template>

<script setup lang="ts">
import NodePanel from './components/NodePanel.vue'
import DesignerCanvas from './components/DesignerCanvas.vue'
import PropertyPanel from './components/PropertyPanel.vue'
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f0f2f5;
}
.editor-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
}
</style>
