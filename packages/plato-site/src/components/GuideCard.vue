<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Hint } from '@/types'
import InlineLatex from './InlineLatex.vue'

const { t } = useI18n()

const props = defineProps<{ guide: Hint }>()

const emit = defineEmits<{
    insert: [tactic: string]
    dismiss: []
    glossaryClick: [term: string]
}>()
</script>

<template>
    <div class="card">
        <InlineLatex :text="guide.text" @glossary-click="emit('glossaryClick', $event)" />
        <br>
        <button v-if="guide.tactic" class="tactic" @click="emit('insert', guide.tactic!)">
            {{ guide.tactic }}
        </button>
        <button v-else class="ok" @click="emit('dismiss')">{{ t('common.ok') }}</button>
    </div>
</template>

<style lang="scss" scoped>
.card {
    width: 100%;
    min-width: 0;
    padding: 10px 14px;
    background: var(--color-subtle-bg);
    border-radius: 8px;
    font-size: 13px;
    line-height: 1.55;
    box-sizing: border-box;
}

.tactic {
    font-family: inherit;
    font-size: 12px;
    margin-top: 6px;
    padding: 4px 12px;
    cursor: pointer;
    background: var(--color-tactic-bg);
    border: 1px solid var(--color-border-light);
    border-radius: 4px;
    color: var(--color-tactic-fg);
    max-width: 80%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.tactic:hover {
    background: var(--color-border-light);
}

.ok {
    font-family: inherit;
    font-size: 12px;
    margin-top: 6px;
    padding: 4px 16px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
}

.ok:hover {
    background: var(--color-primary-hover);
}
</style>
