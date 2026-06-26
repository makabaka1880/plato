<script setup lang="ts">
import { ref, computed, watch, onUnmounted, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { DiscoveryData } from '@/types'
import { useDiscoveryStore } from '@/stores/discovery'
import InlineLatex from '@/components/InlineLatex.vue'
import HelpModal from '@/components/HelpModal.vue'

const props = defineProps<{
    discovery: DiscoveryData
    sectionId: string
}>()

const emit = defineEmits<{
    progress: [idx: number]
    complete: []
    skip: []
}>()

const { t } = useI18n()
const discoveryStore = useDiscoveryStore()

// ── Glossary popup ──────────────────────────────────────────────
const showHelp = ref(false)
const glossaryTerm = ref<string | null>(null)

function onGlossaryClick(term: string) {
    glossaryTerm.value = term
    showHelp.value = true
}

const total = computed(() => props.discovery.lines.length)

// ── Speaker colour palette ──────────────────────────────────────
const SPEAKER_PALETTE = [
    '#4a90d9', '#c06d4f', '#5d9b6c', '#9b6fb0',
    '#d48c3c', '#6d8a9e', '#b8556e', '#4e9b8c',
]

function speakerColor(name: string): string {
    let hash = 0
    for (let i = 0; i < name.length; i++) {
        hash = ((hash << 5) - hash) + name.charCodeAt(i)
        hash |= 0
    }
    return SPEAKER_PALETTE[Math.abs(hash) % SPEAKER_PALETTE.length]!
}

function speakerBgColor(name: string): string {
    const hex = speakerColor(name)
    const r = parseInt(hex.slice(1, 3), 16)
    const g = parseInt(hex.slice(3, 5), 16)
    const b = parseInt(hex.slice(5, 7), 16)
    const a = 20 / 255
    const toHex = (c: number) => Math.round(c * a + 255 * (1 - a)).toString(16).padStart(2, '0')
    return `#${toHex(r)}${toHex(g)}${toHex(b)}`
}

// ── Single-card model with typewriter ───────────────────────────
const TYPING_SPEED = 28
const savedPos = discoveryStore.getPosition(props.sectionId)
const currentIdx = ref(Math.max(0, Math.min(savedPos, total.value - 1)))
const isComplete = computed(() => currentIdx.value >= total.value - 1 && !isTyping.value)
const typedLen = ref(0)
let timer: ReturnType<typeof setInterval> | null = null
let skipTyping = false

onMounted(() => emit('progress', currentIdx.value))
watch(currentIdx, (val) => {
    emit('progress', val)
    discoveryStore.setPosition(props.sectionId, val)
}, { immediate: true })

function fullText(): string {
    if (currentIdx.value < total.value) return props.discovery.lines[currentIdx.value]!.text
    return ''
}

function startTyping() {
    stopTyping()
    typedLen.value = 0
    const target = fullText().length
    if (target === 0) return
    timer = setInterval(() => {
        if (typedLen.value < target) typedLen.value++
        else stopTyping()
    }, TYPING_SPEED)
}

function stopTyping() {
    if (timer !== null) { clearInterval(timer); timer = null }
}

function advance() {
    stopTyping()
    const target = fullText().length
    if (typedLen.value < target) {
        typedLen.value = target
        return
    }
    if (currentIdx.value < total.value) {
        currentIdx.value++
        startTyping()
    }
}

function goPrev() {
    if (currentIdx.value > 0) {
        skipTyping = true
        currentIdx.value--
        stopTyping()
        typedLen.value = fullText().length
    }
}

watch(currentIdx, (val) => {
    if (skipTyping) { skipTyping = false; return }
    if (val > 0 && typedLen.value === 0) {
        typedLen.value = fullText().length
        return
    }
    startTyping()
}, { immediate: true })
onUnmounted(() => stopTyping())

const displayText = computed(() => {
    let t = fullText().slice(0, typedLen.value)
    const dollarCount = (t.match(/\$/g) || []).length
    if (dollarCount % 2 !== 0) t = t.slice(0, t.lastIndexOf('$'))
    const starPairCount = (t.match(/\*\*/g) || []).length
    if (starPairCount % 2 !== 0) t = t.slice(0, t.lastIndexOf('**'))
    const backtickCount = (t.match(/`/g) || []).length
    if (backtickCount % 2 !== 0) t = t.slice(0, t.lastIndexOf('`'))
    let open = 0
    for (let i = 0; i < t.length; i++) {
        if (t[i] === '[') open++
        else if (t[i] === ']') { if (open > 0) open-- }
    }
    if (open > 0) t = t.slice(0, t.lastIndexOf('['))
    return t
})
const isTyping = computed(() => typedLen.value < fullText().length)

const currentLine = computed(() =>
    props.discovery.lines[Math.min(currentIdx.value, total.value - 1)] ?? null,
)

function onKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowDown' || e.key === 'ArrowRight' || e.key === 'Enter' || e.key === ' ') {
        e.preventDefault()
        advance()
    } else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {
        e.preventDefault()
        goPrev()
    }
}
</script>

<template>
    <div class="discovery-dialog" tabindex="0" @keydown="onKeydown" >
        <div class="discovery-header">
            <h2 class="discovery-title">{{ discovery.title }}</h2>
            <button class="header-btn skip-btn" @click.stop="emit('skip')">{{ t('discovery.skip') }}</button>
        </div>

        <div class="stage">
            <!-- Flanking buttons: shown above/below card on narrow screens -->
            <button class="nav-btn nav-flank nav-flank-top" :disabled="currentIdx <= 0" @click.stop="goPrev" title="Previous">▲</button>

            <div class="stage-inner">
                <Transition name="card-fade" mode="out-in">
                    <div
                        v-if="currentLine"
                        :key="currentIdx"
                        class="card"
                        :style="{ backgroundColor: speakerBgColor(currentLine.speaker) }"
                    >
                        <div class="card-speaker" :style="{ color: speakerColor(currentLine.speaker) }">
                            {{ currentLine.speaker }}
                        </div>
                        <div class="card-body">
                            <InlineLatex :text="displayText" @glossary-click="onGlossaryClick" />
                            <span v-if="isTyping" class="cursor">|</span>
                        </div>
                    </div>
                </Transition>

                <!-- Side buttons: shown to the right of card on desktop -->
                <div class="card-nav">
                    <button class="nav-btn nav-side" :disabled="currentIdx <= 0" @click.stop="goPrev" title="Previous">▲</button>
                    <button class="nav-btn nav-side" :disabled="currentIdx >= total - 1" @click.stop="advance" title="Next">▼</button>
                </div>
            </div>

            <button class="nav-btn nav-flank nav-flank-bot" :disabled="currentIdx >= total - 1" @click.stop="advance" title="Next">▼</button>
        </div>

        <div class="bottom-bar">
            <button v-if="isComplete" class="begin-btn" @click.stop="emit('complete')">
                {{ t('discovery.begin') }}
            </button>
        </div>
    </div>

    <HelpModal v-if="showHelp" :glossary-term="glossaryTerm ?? undefined" @close="showHelp = false; glossaryTerm = null" />
</template>

<style lang="scss" scoped>
.discovery-dialog {
    display: flex; flex-direction: column; height: 100%;
    padding: 24px 32px; outline: none;
}

.discovery-header {
    display: flex; align-items: center; margin-bottom: 24px; gap: 12px; flex-shrink: 0;
}
.discovery-title {
    font-size: 20px; font-weight: 400; letter-spacing: -0.01em; margin: 0; color: var(--color-fg);
    flex: 1;
}

.skip-btn {
    font-family: inherit; font-size: 12px; padding: 4px 12px; cursor: pointer;
    background: none; border: 1px solid var(--color-border); border-radius: 4px;
    color: var(--color-muted);
    transition: color 0.15s, border-color 0.15s;
    &:hover { color: var(--color-fg); border-color: var(--color-border-strong); }
}

// ── Stage ────────────────────────────────────────────────────
.stage {
    flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center;
    gap: 0;
}
.stage-inner {
    display: flex; flex-direction: row; align-items: center; justify-content: center;
    gap: 12px;
}
.card {
    width: 520px; flex-shrink: 0; border: 1px solid var(--color-border);
    border-radius: 8px; padding: 16px 22px;
}
.card-speaker {
    font-size: 10px; font-weight: 600; letter-spacing: 0.08em;
    text-transform: uppercase; margin-bottom: 8px;
}
.card-body { font-size: 15px; line-height: 1.65; color: var(--color-fg); }

// ── Side buttons (desktop: right of card) ─────────────────────
.card-nav {
    display: flex; flex-direction: column; gap: 6px; flex-shrink: 0;
}
.nav-side {
    width: 30px; height: 30px; border-radius: 100%;
    font-family: inherit; font-size: 12px; cursor: pointer;
    background: var(--color-subtle-bg); border: 1px solid var(--color-border);
    color: var(--color-muted);
    display: flex; align-items: center; justify-content: center;
    transition: border-color 0.15s, color 0.15s;
    &:hover:not(:disabled) { border-color: var(--color-primary-hover); color: var(--color-primary-hover); }
    &:disabled { opacity: 0.25; cursor: not-allowed; }
}

// ── Flank buttons (narrow: above / below card) ─────────────────
.nav-flank { display: none; }

@media (max-width: 620px) {
    .discovery-dialog { padding: 16px 16px; }
    .stage-inner { gap: 0; }
    .card-nav { display: none; }
    .card { width: 100%; flex-shrink: 1; }
    .nav-flank {
        display: flex; align-items: center; justify-content: center;
        width: 30px; height: 30px; border-radius: 100%;
        font-family: inherit; font-size: 12px; cursor: pointer;
        background: var(--color-subtle-bg); border: 1px solid var(--color-border);
        color: var(--color-muted); flex-shrink: 0;
        transition: border-color 0.15s, color 0.15s;
        &:hover:not(:disabled) { border-color: var(--color-primary-hover); color: var(--color-primary-hover); }
        &:disabled { opacity: 0.25; cursor: not-allowed; }
    }
    .nav-flank-top { margin-bottom: 8px; }
    .nav-flank-bot { margin-top: 8px; }
}

// ── Typewriter cursor ─────────────────────────────────────────
.cursor {
    display: inline-block; font-weight: 300;
    color: var(--color-primary-hover); animation: blink 0.8s step-end infinite;
}
@keyframes blink { 0%, 100% { opacity: 1; } 50% { opacity: 0; } }

// ── Bottom bar ────────────────────────────────────────────────
.bottom-bar {
    flex-shrink: 0; display: flex; justify-content: center;
    align-items: center; height: 56px;
}
.begin-btn {
    font-family: inherit; font-size: 16px; padding: 10px 40px; cursor: pointer;
    background: var(--color-primary); color: var(--color-primary-fg);
    border: none; border-radius: 6px; transition: background 0.15s;
    &:hover { background: var(--color-primary-hover); }
}

// ── Card transitions ──────────────────────────────────────────
.card-fade-enter-active { transition: opacity 0.35s ease; }
.card-fade-leave-active { transition: opacity 0.25s ease; }
.card-fade-enter-from,
.card-fade-leave-to { opacity: 0; }
</style>
