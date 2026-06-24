<script setup lang="ts">
import katex from 'katex'
import { computed } from 'vue'

const props = defineProps<{ expr: string }>()

const html = computed(() => {
  try {
    return katex.renderToString(props.expr, { throwOnError: false, displayMode: false })
  } catch {
    return props.expr
  }
})
</script>

<template>
  <span v-html="html"></span>
</template>
