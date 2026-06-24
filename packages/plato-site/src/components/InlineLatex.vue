<script setup lang="ts">
import { computed } from 'vue'
import Katex from '@/components/Katex.vue'
import { parseInline } from '@/utils/inline-parse'

const props = defineProps<{ text: string }>()

const segments = computed(() => parseInline(props.text))
</script>

<template>
  <template v-for="seg in segments" :key="seg.key">
    <Katex v-if="seg.type === 'latex'" :expr="seg.value" />
    <b v-else-if="seg.type === 'bold'">{{ seg.value }}</b>
    <code v-else-if="seg.type === 'code'" class="inline-code">{{ seg.value }}</code>
    <span v-else>{{ seg.value }}</span>
  </template>
</template>

<style scoped>
.inline-code {
  font-family: inherit;
  background: var(--color-border);
  padding: 1px 5px;
  border-radius: 3px;
  font-size: 0.92em;
}
</style>
