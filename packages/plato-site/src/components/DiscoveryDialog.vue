<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { DiscoveryData } from '@/types'
import { usePreferencesStore } from '@/stores/preferences'
import InlineLatex from '@/components/InlineLatex.vue'

const props = defineProps<{
    discovery: DiscoveryData
    sectionId: string
}>()

const emit = defineEmits<{
    complete: []
    skip: []
}>()

const { t, locale } = useI18n()
const prefs = usePreferencesStore()

function setLocale(loc: string) {
    locale.value = loc
    prefs.setLocale(loc)
}

const total = computed(() => props.discovery.lines.length)

// ── Speaker colour palette ──────────────────────────────────────────
const SPEAKER_PALETTE = [
    '#4a90d9', // blue
    '#c06d4f', // terracotta
    '#5d9b6c', // sage green
    '#9b6fb0', // lavender
    '#d48c3c', // amber
    '#6d8a9e', // slate
    '#b8556e', // rose
    '#4e9b8c', // teal
]

/** Deterministic hash — same speaker always gets the same colour index. */
function speakerColor(name: string): string {
    let hash = 0
    for (let i = 0; i < name.length; i++) {
        hash = ((hash << 5) - hash) + name.charCodeAt(i)
        hash |= 0
    }
    return SPEAKER_PALETTE[Math.abs(hash) % SPEAKER_PALETTE.length]!
}

// ── Conveyor-belt card model ────────────────────────────────────────
// Phases:
//   'enter'  – card just appeared; sits below the bottom slot
//   'active' – card at its target position (fully visible)
//   'exit'   – card sliding up off-screen

interface CardState {
    lineIdx: number
    pos: number       // 0 = top, 1 = bottom
    phase: 'enter' | 'active' | 'exit'
}

const cards = ref<CardState[]>([])
const currentIdx = ref(-1)
const isComplete = computed(() => currentIdx.value >= total.value)
let transitionBusy = false

// Show first line immediately on mount
onMounted(() => {
    if (total.value > 0) {
        currentIdx.value = 0
        pushCard(0)
    }
})

function advance() {
    if (transitionBusy) return

    if (currentIdx.value < total.value) {
        currentIdx.value++
        pushCard(currentIdx.value)
        return
    }
    if (isComplete.value) {
        emit('complete')
    }
}

function doSkip() {
    emit('skip')
}

// ── Conveyor logic ──────────────────────────────────────────────────
function pushCard(lineIdx: number) {
    transitionBusy = true

    for (const c of cards.value) {
        if (c.phase === 'active') {
            if (c.pos === 0) {
                c.phase = 'exit'
            } else {
                c.pos = 0
            }
        }
    }

    cards.value.push({ lineIdx, pos: 1, phase: 'enter' })

    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            const latest = cards.value[cards.value.length - 1]
            if (latest) latest.phase = 'active'

            setTimeout(() => {
                cards.value = cards.value.filter(c => c.phase !== 'exit')
                transitionBusy = false
            }, 480)
        })
    })
}

// ── Visual positioning ──────────────────────────────────────────────
const TOP_CENTRE    = 'calc(50% - 46px)'
const BOTTOM_CENTRE = 'calc(50% + 46px)'
const BELOW         = 'calc(50% + 100px)'
const ABOVE         = 'calc(50% - 106px)'

function cardTop(pos: number, phase: string): string {
    if (phase === 'enter') return BELOW
    if (phase === 'exit')  return ABOVE
    return pos === 0 ? TOP_CENTRE : BOTTOM_CENTRE
}

function cardOpacity(phase: string): string {
    return phase === 'enter' || phase === 'exit' ? '0' : '1'
}
</script>

<template>
    <div class="discovery-dialog" @click="advance">
        <div class="discovery-header">
            <h2 class="discovery-title">{{ discovery.title }}</h2>
            <div class="lang-switch" @click.stop>
                <button :class="{ active: locale === 'en' }" @click.stop="setLocale('en')">EN</button>
                <button :class="{ active: locale === 'zh' }" @click.stop="setLocale('zh')">中文</button>
            </div>
            <button class="skip-btn" @click.stop="doSkip">{{ t('discovery.skip') }}</button>
        </div>

        <div class="stage">
            <div
                v-for="card in cards"
                :key="card.lineIdx"
                class="card"
                :style="{
                    top: cardTop(card.pos, card.phase),
                    opacity: cardOpacity(card.phase),
                    backgroundColor: speakerColor(discovery.lines[card.lineIdx]!.speaker) + '14',
                }"
            >
                <div class="card-speaker" :style="{ color: speakerColor(discovery.lines[card.lineIdx]!.speaker) }">
                    {{ discovery.lines[card.lineIdx]!.speaker }}
                </div>
                <div class="card-body">
                    <InlineLatex :text="discovery.lines[card.lineIdx]!.text" />
                </div>
            </div>
        </div>

        <div class="bottom-bar">
            <button
                v-if="isComplete"
                class="begin-btn"
                @click.stop="emit('complete')"
            >
                {{ t('discovery.begin') }}
            </button>
            <div v-else-if="!isComplete" class="tap-hint">{{ t('discovery.tapToContinue') }}</div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
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
    align-items: center;
    margin-bottom: 24px;
    gap: 12px;
    flex-shrink: 0;
}

.discovery-title {
    font-size: 20px;
    font-weight: 400;
    letter-spacing: -0.01em;
    margin: 0;
    color: var(--color-fg);
}

.lang-switch {
    display: flex;
    gap: 2px;
    margin-left: auto;
}

.lang-switch button {
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--color-muted);
    background: none;
    border: 1px solid transparent;
    border-radius: 3px;
    padding: 3px 8px;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
}

.lang-switch button:hover {
    color: var(--color-fg);
    border-color: var(--color-border);
}

.lang-switch button.active {
    color: var(--color-fg);
    border-color: var(--color-border);
    background: var(--color-subtle-bg);
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

/* ── stage & cards ────────────────── */
.stage {
    flex: 1;
    position: relative;
    overflow: hidden;
    min-height: 200px;
}

.card {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    max-width: 520px;
    background: var(--color-subtle-bg);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 16px 22px;
    transition: top 0.45s cubic-bezier(0.4, 0, 0.2, 1),
                opacity 0.45s cubic-bezier(0.4, 0, 0.2, 1),
                background 0.45s cubic-bezier(0.4, 0, 0.2, 1);
    pointer-events: none;
}

.card-speaker {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    margin-bottom: 8px;
    transition: color 0.45s cubic-bezier(0.4, 0, 0.2, 1);
}

.card-body {
    font-size: 15px;
    line-height: 1.65;
    color: var(--color-fg);
}

/* ── bottom bar ───────────────────── */
.bottom-bar {
    flex-shrink: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 56px;
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

.tap-hint {
    font-size: 13px;
    color: var(--color-muted);
    animation: pulse-text 1.8s ease-in-out infinite;
}

@keyframes pulse-text {
    0%, 100% { opacity: 0.35; }
    50%      { opacity: 1.0; }
}
</style>
