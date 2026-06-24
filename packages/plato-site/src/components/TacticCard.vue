<script setup lang="ts">
import type { Tactic } from '@/types'
import Katex from './Katex.vue'
import InlineLatex from './InlineLatex.vue'

defineProps<{
  tactic: Tactic
  animate: boolean
}>()
</script>

<template>
  <div class="card" :class="{ pop: animate }">
    <div class="name">{{ tactic.name }}</div>
    <div class="rule">
      <Katex :expr="'\\displaystyle ' + tactic.rule" />
    </div>
    <div class="desc">
      <InlineLatex :text="tactic.description" />
    </div>
  </div>
</template>

<style scoped>
.card {
  padding: 12px 14px; background: var(--color-card-bg);
  border-radius: 8px; border: 1px solid var(--color-border);
  font-size: 13px; line-height: 1.5;
  transition: all 0.3s ease;
}
.card + .card { margin-top: 8px; }

.name {
  font-weight: 600; font-size: 14px; margin-bottom: 8px;
}
.rule {
  display: flex; justify-content: center;
  padding: 12px 0; font-size: 15px;
}
.desc {
  font-size: 12px; color: var(--color-subtle); margin-top: 8px;
}

.pop {
  animation: cardPop 0.4s ease;
}
@keyframes cardPop {
  0%   { transform: scale(0.95); opacity: 0.5; }
  60%  { transform: scale(1.02); }
  100% { transform: scale(1); opacity: 1; }
}
</style>
