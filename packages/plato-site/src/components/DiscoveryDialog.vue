<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { DiscoveryData } from '@/types'
import InlineLatex from '@/components/InlineLatex.vue'

const props = defineProps<{
    discovery: DiscoveryData
    sectionId: string
}>()

const emit = defineEmits<{
    complete: []
    skip: []
}>()

const { t } = useI18n()

const currentIdx = ref(0)
const total = computed(() => props.discovery.lines.length)
const isComplete = computed(() => currentIdx.value >= total.value)

function advance() {
    if (currentIdx.value < total.value) {
        currentIdx.value++
        // Don't auto-complete on the final line reveal — let user
        // read it first, then click "Begin" or tap again to finish.
        return
    }
    // All lines already revealed — another click confirms completion.
    if (isComplete.value) {
        emit('complete')
    }
}

function doSkip() {
    emit('skip')
}

// Character label helpers
const currentLine = computed(() =>
    currentIdx.value < total.value ? props.discovery.lines[currentIdx.value] : null,
)

function speakerLabel(speaker: string): string {
    return speaker === 'plato' ? t('discovery.plato') : t('discovery.aristotle')
}
</script>

<template>
    <div class="discovery-dialog" @click="advance">
        <div class="discovery-header">
            <h2 class="discovery-title">{{ discovery.title }}</h2>
            <button class="skip-btn" @click.stop="doSkip">{{ t('discovery.skip') }}</button>
        </div>

        <div class="conversation">
            <template v-for="(line, i) in discovery.lines" :key="i">
                <Transition name="line-fade">
                    <div
                        v-if="i <= currentIdx"
                        class="line-row"
                        :class="line.speaker"
                    >
                        <div class="speaker-label">{{ speakerLabel(line.speaker) }}</div>
                        <div class="bubble" :class="line.speaker">
                            <InlineLatex :text="line.text" />
                        </div>
                    </div>
                </Transition>
            </template>
        </div>

        <Transition name="fade-up">
            <div v-if="isComplete" class="begin-row">
                <button class="begin-btn" @click.stop="emit('complete')">
                    {{ t('discovery.begin') }}
                </button>
            </div>
        </Transition>

        <div v-if="!isComplete" class="tap-hint">
            {{ currentIdx < total ? '▼' : '' }}
        </div>
    </div>
</template>

<style scoped>
.discovery-dialog {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 24px 16px;
    cursor: pointer;
    user-select: none;
}

.discovery-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 24px;
    gap: 12px;
}

.discovery-title {
    font-size: 20px;
    font-weight: 400;
    letter-spacing: -0.01em;
    margin: 0;
    color: var(--color-fg);
}

.skip-btn {
    font-family: inherit;
    font-size: 12px;
    padding: 4px 12px;
    cursor: pointer;
    background: none;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-muted);
    flex-shrink: 0;
    transition: color 0.15s, border-color 0.15s;
}

.skip-btn:hover {
    color: var(--color-fg);
    border-color: var(--color-border-strong);
}

/* ── conversation ─────────────────── */
.conversation {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-right: 8px;
}

.line-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-width: 85%;
}

.line-row.plato {
    align-self: flex-start;
}

.line-row.aristotle {
    align-self: flex-end;
}

.speaker-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.06em;
    color: var(--color-border-strong);
    padding: 0 12px;
}

.line-row.aristotle .speaker-label {
    text-align: right;
}

.bubble {
    padding: 12px 16px;
    border-radius: 12px;
    font-size: 15px;
    line-height: 1.6;
    color: var(--color-fg);
}

.bubble.plato {
    background: var(--color-subtle-bg);
    border: 1px solid var(--color-border);
    border-bottom-left-radius: 4px;
}

.bubble.aristotle {
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border-bottom-right-radius: 4px;
}

.bubble.aristotle :deep(.glossary-link) {
    color: var(--color-primary-fg);
    text-decoration-color: var(--color-primary-fg);
    opacity: 0.85;
}

/* ── begin ────────────────────────── */
.begin-row {
    display: flex;
    justify-content: center;
    padding: 24px 0 8px;
}

.begin-btn {
    font-family: inherit;
    font-size: 16px;
    padding: 10px 40px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 6px;
    transition: background 0.15s;
}

.begin-btn:hover {
    background: var(--color-primary-hover);
}

/* ── tap hint ─────────────────────── */
.tap-hint {
    display: flex;
    justify-content: center;
    padding: 16px 0 0;
    font-size: 18px;
    color: var(--color-border);
    animation: bounce 1.2s ease infinite;
}

@keyframes bounce {
    0%, 100% { transform: translateY(0); opacity: 0.4; }
    50% { transform: translateY(6px); opacity: 1; }
}

/* ── transitions ──────────────────── */
.line-fade-enter-active {
    transition: all 0.4s ease;
}

.line-fade-enter-from {
    opacity: 0;
    transform: translateY(12px);
}

.fade-up-enter-active {
    transition: all 0.5s ease;
}

.fade-up-enter-from {
    opacity: 0;
    transform: translateY(16px);
}
</style>
