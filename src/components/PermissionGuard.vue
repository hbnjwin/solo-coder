<template>
  <!-- FIXME: slot content may flash before permission check completes -->
  <div v-if="permitted" class="permission-guard">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { usePermissions } from '../composables/usePermissions'

const props = defineProps<{
  permission: string
}>()

const { hasPermission } = usePermissions()

const permitted = computed(() => hasPermission(props.permission))
</script>

<style scoped>
.permission-guard {
  display: contents;
}
</style>
