<script setup lang="ts">
import { computed } from 'vue'
import type { Segment } from '@/utils/inline-parse'
import Katex from '@/components/Katex.vue'
import { parseInline } from '@/utils/inline-parse'

const props = defineProps<{ text: string }>()

const emit = defineEmits<{ glossaryClick: [term: string] }>()

const segments = computed(() => parseInline(props.text))

/** Render a single segment — used both at top level and recursively inside bold. */
function renderSeg(seg: Segment) {
  switch (seg.type) {
    case 'latex': return { type: 'latex' as const, expr: seg.value }
    case 'bold': return { type: 'bold' as const, children: seg.children }
    case 'code': return { type: 'code' as const, code: seg.value }
    case 'glossary': return { type: 'glossary' as const, id: seg.value, label: seg.display ?? seg.value }
    case 'text': return { type: 'text' as const, value: seg.value }
  }
}
</script>

<template>
  <template v-for="seg in segments" :key="seg.key">
    <Katex v-if="seg.type === 'latex'" :expr="seg.value" />
    <b v-else-if="seg.type === 'bold' && !seg.children">{{ seg.value }}</b>
    <b v-else-if="seg.type === 'bold' && seg.children">
      <template v-for="child in seg.children" :key="child.key">
        <Katex v-if="child.type === 'latex'" :expr="child.value" />
        <b v-else-if="child.type === 'bold' && !child.children">{{ child.value }}</b>
        <b v-else-if="child.type === 'bold' && child.children">
          <template v-for="c2 in child.children" :key="c2.key">
            <Katex v-if="c2.type === 'latex'" :expr="c2.value" />
            <code v-else-if="c2.type === 'code'" class="inline-code">{{ c2.value }}</code>
            <span v-else-if="c2.type === 'glossary'" class="glossary-link" @click="emit('glossaryClick', c2.value)">{{ c2.display ?? c2.value }}</span>
            <span v-else>{{ c2.value }}</span>
          </template>
        </b>
        <code v-else-if="child.type === 'code'" class="inline-code">{{ child.value }}</code>
        <span v-else-if="child.type === 'glossary'" class="glossary-link" @click="emit('glossaryClick', child.value)">{{ child.display ?? child.value }}</span>
        <span v-else>{{ child.value }}</span>
      </template>
    </b>
    <code v-else-if="seg.type === 'code'" class="inline-code">{{ seg.value }}</code>
    <span v-else-if="seg.type === 'glossary'" class="glossary-link" @click="emit('glossaryClick', seg.value)">{{ seg.display ?? seg.value }}</span>
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
.glossary-link {
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  cursor: pointer;
  color: var(--color-primary-hover);
  transition: color 0.15s;
}
.glossary-link:hover {
  color: var(--color-fg);
}
</style>
