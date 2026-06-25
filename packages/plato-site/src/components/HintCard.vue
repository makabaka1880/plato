<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Hint } from '@/types'
import InlineLatex from './InlineLatex.vue'

const { t } = useI18n()

const props = defineProps<{ hint: Hint; pulse: boolean }>()

const emit = defineEmits<{
  insert: [tactic: string]
  dismiss: []
  glossaryClick: [term: string]
}>()

function onTactic(tactic: string) {
  emit('insert', tactic)
  emit('dismiss')
}
</script>

<template>
  <div class="card">
    <button class="close" @click="emit('dismiss')" :title="t('common.dismiss')">{{ t('common.close') }}</button>
    <InlineLatex :text="hint.text" @glossary-click="emit('glossaryClick', $event)" />
    <button
      v-if="hint.tactic"
      class="tactic"
      :class="{ pulse }"
      @click="onTactic(hint.tactic!)"
    >
      {{ hint.tactic }}
    </button>
    <button v-else ref="okBtn" class="ok small" @click="emit('dismiss')">{{ t('common.ok') }}</button>
  </div>
</template>

<style scoped>
.card {
  position: relative; width: min(80vw, 520px);
  padding: 10px 14px; margin-bottom: 6px;
  background: var(--color-hint-bg); border-radius: 8px;
  font-size: 13px; line-height: 1.55; border: 1px solid var(--color-hint-border);
}
.close {
  position: absolute; top: 6px; right: 10px;
  background: none; border: none; font-size: 16px;
  cursor: pointer; color: var(--color-muted); line-height: 1;
}
.close:hover { color: var(--color-primary-hover); }

.tactic {
  font-family: inherit; font-size: 12px; margin-top: 6px;
  padding: 4px 12px; cursor: pointer;
  background: var(--color-tactic-bg); border: 1px solid var(--color-border-light);
  border-radius: 4px; color: var(--color-tactic-fg);
  max-width: 80%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.tactic:hover { background: var(--color-border-light); }

.ok {
  font-family: inherit; font-size: 12px; margin-top: 6px;
  padding: 4px 16px; cursor: pointer;
  background: var(--color-primary); color: var(--color-primary-fg); border: none;
  border-radius: 4px;
}
.ok.small { padding: 2px 12px; font-size: 11px; }
.ok:hover { background: var(--color-primary-hover); }

.pulse { animation: btnGlow 1s ease-in-out infinite; }

@keyframes btnGlow {
  0%, 100% { background: var(--color-tactic-bg); }
  50%      { background: #d0d0ff; border-color: #aac; }
}
</style>
