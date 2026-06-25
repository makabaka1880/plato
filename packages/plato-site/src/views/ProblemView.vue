<script setup lang="ts">
import { computed, ref, watch, nextTick, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Problem } from '@/types'
import { useProblemLatex } from '@/composables/useProblemLatex'
import { useVictory } from '@/composables/useVictory'
import { useProgressStore } from '@/stores/progress'
import { useRoadmapStore } from '@/stores/roadmap'
import Katex from '@/components/Katex.vue'
import InlineLatex from '@/components/InlineLatex.vue'
import ProofRepl from '@/components/ProofRepl.vue'
import PreferenceModal from '@/components/PreferenceModal.vue'
import TacticSidebar from '@/components/TacticSidebar.vue'
import RoadmapModal from '@/components/RoadmapModal.vue'

const { t } = useI18n()

const props = defineProps<{
    problemIdx: number
    problems: Problem[]
}>()

const emit = defineEmits<{
    next: []
    prev: []
    home: []
}>()

const progress = useProgressStore()
const roadmap = useRoadmapStore()

const problem = computed(() => props.problems[props.problemIdx] ?? null)
const hasNext = computed(() =>
    props.problemIdx < props.problems.length - 1 &&
    props.problemIdx < progress.highestSolved + 1
)
const hasPrev = computed(() => props.problemIdx > 0)

const problemRef = computed(() => problem.value)
const { goalLatex, premiseLatex, updateLatex } = useProblemLatex(problemRef)
const victory = useVictory()

const prefsOpen = ref(false)
const agreed = ref(false)
const showRepl = ref(false)
const proofLines = ref<string[]>([])
const roadmapOpen = ref(false)

const sortedEntries = computed(() =>
    [...roadmap.entries].sort((a, b) => a.idx - b.idx)
)

watch(() => props.problemIdx, () => {
    agreed.value = false
    showRepl.value = false
    proofLines.value = []
    victory.solved.value = false
    updateLatex()
})

async function onAgree() {
    agreed.value = true
    await nextTick()
    setTimeout(() => { showRepl.value = true }, 500)
}

function onSolved(lines: string[]) {
    proofLines.value = lines
    if (problem.value) {
        victory.fire(problem.value.unlocks)
        progress.markSolved(props.problemIdx)
        roadmap.add({
            idx: props.problemIdx,
            description: problem.value.description,
            goal: problem.value.goal,
            proofLines: lines,
        })
    }
}

</script>

<template>
    <div v-if="!problem" class="not-found">
        {{ t('problem.notFound') }}
    </div>
    <div v-else class="root-row">
        <div class="root">
            <div class="header">
                <button class="logo" @click="emit('home')">{{ t('problem.logo') }}</button>
                <span class="spacer"></span>
                <span class="goal-chip">{{ problem.goal }}</span>
                <span class="spacer"></span>
                <a class="gh-link" href="https://github.com/makabaka1880/plato" target="_blank"
                    title="GitHub">GitHub</a>
                <button class="prefs-link" @click="prefsOpen = true">{{ t('problem.preferences') }}</button>
            </div>

            <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />

            <div class="body">
                <Transition name="fade-up">
                    <div v-if="!agreed" class="prompt">
                        <div class="prove-label">{{ t('problem.makeMeBelieve') }}</div>
                        <div class="prove-desc">
                            <InlineLatex :text="problem.description" />
                        </div>
                        <div class="goal-line">
                            <span v-if="problem.premise.length" class="premise-label">{{ t('problem.premise') }}</span>
                            <span v-if="problem.premise.length" class="premise-katex">
                                <Katex :expr="premiseLatex" />
                            </span>
                            <span class="goal-label">{{ t('problem.goal') }}</span>
                            <span class="goal-katex">
                                <Katex :expr="goalLatex" />
                            </span>
                        </div>
                        <button class="agree-btn" @click="onAgree">{{ t('problem.agree') }}</button>
                    </div>
                </Transition>

                <Transition name="fade-in">
                    <ProofRepl v-if="agreed && showRepl" :goal-latex="goalLatex" :premise-latex="premiseLatex"
                        :goal="problem.goal" :premise="problem.premise" :guides="problem.guides" :hints="problem.hints"
                        @solved="onSolved" style="flex:1;overflow:hidden" />
                </Transition>

                <Transition name="fade-in">
                    <div v-if="victory.solved.value" class="victory-overlay">
                        <div class="victory">
                            <div class="victory-text">{{ t('problem.victory') }}</div>
                            <div v-if="proofLines.length" class="proof-scroll">
                                <div v-for="(line, i) in proofLines" :key="i" class="proof-line">
                                    <InlineLatex :text="line" />
                                </div>
                            </div>
                            <button v-if="hasNext" class="victory-btn" @click="emit('next')">
                                {{ t('problem.nextProblem') }}
                            </button>
                            <button v-else class="victory-btn" @click="emit('home')">
                                {{ t('problem.backHome') }}
                            </button>
                        </div>
                    </div>
                </Transition>
            </div>

            <div class="footer">
                <button @click="emit('prev')" :disabled="!hasPrev" class="nav-btn">{{ t('problem.prev') }}</button>
                <div class="footer-roadmap" @click="roadmapOpen = true">
                    <div class="mini-track">
                        <div v-for="(entry, i) in sortedEntries" :key="entry.idx" class="mini-dot"
                            :class="{ latest: i === sortedEntries.length - 1 }">
                            <span class="mini-dot-num">{{ i + 1 }}</span>
                        </div>
                    </div>
                </div>
                <button @click="emit('next')" :disabled="!hasNext" class="nav-btn">{{ t('problem.next') }}</button>
            </div>
        </div>

        <TacticSidebar />
        <RoadmapModal v-if="roadmapOpen" :total-problems="problems.length" @close="roadmapOpen = false" />
    </div>
</template>

<style scoped>
.root-row {
    display: flex;
    flex-direction: row;
    height: 100%;
}

.root {
    display: flex;
    flex-direction: column;
    height: 100%;
    flex: 1;
    overflow: hidden;
}

.header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    border-bottom: 1px solid var(--color-border);
    font-size: 15px;
}

.logo {
    text-decoration: none;
    font-weight: 600;
    color: inherit;
    border: none;
    background: none;
    cursor: pointer;
    font-family: inherit;
    font-size: inherit;
    padding: 0;
}

.sep {
    color: var(--color-border-strong);
}

.spacer {
    flex: 1;
}

.goal-chip {
    font-size: 13px;
    color: var(--color-muted);
}

.prefs-link {
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    background: none;
    border: none;
    color: var(--color-muted);
    padding: 2px 6px;
    border-radius: 4px;
}

.prefs-link:hover {
    color: var(--color-fg);
    background: var(--color-subtle-bg);
}

.gh-link {
    font-family: inherit;
    font-size: 13px;
    color: var(--color-muted);
    text-decoration: none;
    padding: 2px 6px;
    border-radius: 4px;
}

.gh-link:hover {
    color: var(--color-fg);
    background: var(--color-subtle-bg);
}

.body {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
}

.footer {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-top: 1px solid var(--color-border);
}

.footer-roadmap {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    cursor: pointer;
    transition: opacity 0.15s;
    min-width: 0;
}

.footer-roadmap:hover {
    opacity: 0.7;
}

.nav-btn {
    padding: 4px 12px;
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
}

.mini-track {
    display: flex;
    gap: 4px;
    justify-content: center;
    flex-wrap: wrap;
}

.mini-dot {
    width: 22px;
    height: 22px;
    border-radius: 100%;
    background: var(--color-subtle-bg);
    border: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    animation: dotAppear 0.4s ease both;
}

.mini-dot:nth-child(1) {
    animation-delay: 0s;
}

.mini-dot:nth-child(2) {
    animation-delay: 0.05s;
}

.mini-dot:nth-child(3) {
    animation-delay: 0.1s;
}

.mini-dot:nth-child(4) {
    animation-delay: 0.15s;
}

.mini-dot:nth-child(5) {
    animation-delay: 0.2s;
}

.mini-dot:nth-child(6) {
    animation-delay: 0.25s;
}

.mini-dot.latest {
    background: var(--color-primary);
    border-color: var(--color-primary);
}

.mini-dot-num {
    font-size: 10px;
    font-weight: 600;
    color: var(--color-muted);
}

.mini-dot.latest .mini-dot-num {
    color: var(--color-primary-fg);
}

.mini-label {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--color-border-strong);
}

@keyframes dotAppear {
    0% {
        transform: scale(0);
        opacity: 0;
    }

    60% {
        transform: scale(1.15);
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

.not-found {
    padding: 32px;
}

/* ── prompt ─────────────────────── */
.prompt {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 40px 20px;
}

.prove-label {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.1em;
    color: var(--color-muted);
    margin-bottom: 16px;
}

.prove-desc {
    font-size: clamp(20px, 4vw, 32px);
    max-width: 700px;
    line-height: 1.45;
    margin-bottom: 32px;
}

.goal-line {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 15px;
    color: var(--color-subtle);
    margin-bottom: 32px;
}

.premise-label {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: var(--color-border-strong);
}

.premise-katex {
    font-size: 16px;
    color: var(--color-muted);
    margin-right: 16px;
}

.goal-label {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: #aaa;
}

.goal-katex {
    font-size: 18px;
    color: var(--color-primary-hover);
}

.agree-btn {
    font-family: inherit;
    font-size: 14px;
    padding: 8px 32px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
}

.agree-btn:hover {
    background: var(--color-primary-hover);
}

/* ── victory ───────────────────── */
.victory-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.92);
    z-index: 10;
}

.victory {
    text-align: center;
    max-height: 90vh;
    overflow-y: auto;
    padding: 20px 16px;
}

.victory-text {
    font-size: clamp(28px, 5vw, 48px);
    font-weight: 400;
    margin-bottom: 16px;
}

.proof-scroll {
    max-height: 35vh;
    overflow-y: auto;
    text-align: left;
    font-size: 13px;
    line-height: 2;
    color: var(--color-subtle);
    padding: 16px 18px;
    margin: 0 auto 24px;
    background: var(--color-subtle-bg);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    width: min(560px, 85vw);
    white-space: pre-wrap;
}

.victory-btn {
    font-family: inherit;
    font-size: 15px;
    padding: 8px 28px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
    display: inline-block;
}

.victory-btn:hover {
    background: var(--color-primary-hover);
}

/* ── transitions ────────────────── */
.fade-up-enter-active {
    transition: all 0.4s ease;
}

.fade-up-leave-active {
    transition: all 0.4s ease;
}

.fade-up-enter-from {
    opacity: 0;
    transform: translateY(12px);
}

.fade-up-leave-to {
    opacity: 0;
    transform: translateY(-12px);
}

.fade-in-enter-active {
    transition: opacity 0.4s ease 0.3s;
}

.fade-in-leave-active {
    transition: opacity 0.15s ease;
}

.fade-in-enter-from {
    opacity: 0;
}

.fade-in-leave-to {
    opacity: 0;
}
</style>
