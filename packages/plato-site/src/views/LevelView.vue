<script setup lang="ts">
import { computed, ref, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getSection, getNextSection } from '@/data'
import { useProblemLatex } from '@/composables/useProblemLatex'
import { useVictory } from '@/composables/useVictory'
import { useProgressStore } from '@/stores/progress'
import { useRoadmapStore } from '@/stores/roadmap'
import { renderNlg } from '@/composables/useNlg'
import type { StepMeta } from '@/composables/useNlg'
import { loadNlg } from '@/data'
import { useDiscoveryStore } from '@/stores/discovery'
import type { Problem } from '@/types'
import Katex from '@/components/Katex.vue'
import InlineLatex from '@/components/InlineLatex.vue'
import ProofRepl from '@/components/ProofRepl.vue'
import PreferenceModal from '@/components/PreferenceModal.vue'
import TacticSidebar from '@/components/TacticSidebar.vue'
import RoadmapModal from '@/components/RoadmapModal.vue'
import NavBar from '@/components/NavBar.vue'
import HelpModal from '@/components/HelpModal.vue'
import DiscoveryDialog from '@/components/DiscoveryDialog.vue'

const router = useRouter()
const { t, locale } = useI18n()

const props = defineProps<{
    sectionId: string
    levelIdx: number
}>()

const progress = useProgressStore()
const roadmap = useRoadmapStore()
const discovery = useDiscoveryStore()

// ── Section & level resolution ────────────────────────────────────────
const section = computed(() => getSection(props.sectionId, locale.value))
const level = computed(() => section.value?.levels[props.levelIdx] ?? null)
const isDiscovery = computed(() => level.value?.type === 'discovery')
const problem = computed(() => level.value?.type === 'problem' ? level.value.data as Problem : null)
const discoveryData = computed(() => level.value?.type === 'discovery' ? level.value.data : null)
const sectionName = computed(() => {
    if (!section.value) return ''
    return t(section.value.meta.nameI18nKey)
})

// ── Locking guard ──────────────────────────────────────────────────────
watch(
    () => [props.sectionId, props.levelIdx] as const,
    ([sectionId, idx]) => {
        const sec = getSection(sectionId, locale.value)
        if (!sec) {
            router.replace({ name: 'notFound' })
            return
        }
        if (!Number.isFinite(idx) || idx < 0 || idx >= sec.levels.length) {
            router.replace({ name: 'notFound' })
            return
        }
        if (!progress.isUnlocked(sectionId, idx)) {
            const next = (progress.highestCompletedLevel[sectionId] ?? -1) + 1
            router.replace({
                name: 'locked',
                query: {
                    section: sectionId,
                    n: String(idx),
                    closest: String(next),
                },
            })
            return
        }
    },
    { immediate: true },
)

// Re-check when locale changes
watch(() => section.value?.levels.length, (len) => {
    if (len !== undefined && (props.levelIdx < 0 || props.levelIdx >= len)) {
        router.replace({ name: 'notFound' })
    }
})

// ── Navigation ─────────────────────────────────────────────────────────
const hasPrev = computed(() => props.levelIdx > 0)
const hasNext = computed(() => {
    if (!section.value) return false
    const nextIdx = props.levelIdx + 1
    if (nextIdx >= section.value.levels.length) return false
    return progress.isUnlocked(props.sectionId, nextIdx)
})

function goPrev() {
    router.push(`/section/${props.sectionId}/level/${props.levelIdx - 1}`)
}

function goNext() {
    router.push(`/section/${props.sectionId}/level/${props.levelIdx + 1}`)
}

// After completing the last level of a section, check for next section
function goNextSection() {
    const next = getNextSection(props.sectionId, locale.value)
    if (next) {
        router.push(`/section/${next.id}/level/0`)
    } else {
        router.push('/')
    }
}

// ── Discovery callbacks ────────────────────────────────────────────────
const discoveryIdx = ref(0)

function onDiscoveryProgress(idx: number) {
    discoveryIdx.value = idx
    progress.saveCurrentProgress({
        sectionId: props.sectionId,
        levelIdx: props.levelIdx,
        tag: 'discovery',
        idx,
    })
}

function onStepTaken(steps: StepMeta[]) {
    progress.saveCurrentProgress({
        sectionId: props.sectionId,
        levelIdx: props.levelIdx,
        tag: 'problem',
        proof: steps,
    })
}

function onSolved(steps: StepMeta[]) {
    progress.saveCurrentProgress({
        sectionId: props.sectionId,
        levelIdx: props.levelIdx,
        tag: 'problem',
        proof: steps,
    })
    if (problem.value) {
        victory.fire(problem.value.unlocks)
        progress.markCompleted(props.sectionId, props.levelIdx)
        roadmap.add({
            sectionId: props.sectionId,
            sectionIdx: props.levelIdx,
            tag: 'problem',
            goal: problem.value.goal,
            proof: steps,
        })
        // Auto-transition to next section if this was the last level
        if (section.value && props.levelIdx === section.value.levels.length - 1) {
            const next = getNextSection(props.sectionId, locale.value)
            if (next) {
                router.push(`/section/${next.id}/level/0`)
            }
        }
    }
}

function onDiscoveryComplete() {
    progress.markCompleted(props.sectionId, props.levelIdx)
    discovery.markViewed(props.sectionId, props.levelIdx)
    const totalLines = discoveryData.value?.lines.length ?? 0
    roadmap.add({
        sectionId: props.sectionId,
        sectionIdx: props.levelIdx,
        tag: 'discovery',
        idx: totalLines > 0 ? totalLines - 1 : 0,
    })
    if (hasNext.value) goNext()
    else goNextSection()
}

function onDiscoverySkip() {
    progress.markCompleted(props.sectionId, props.levelIdx)
    discovery.markViewed(props.sectionId, props.levelIdx)
    roadmap.add({
        sectionId: props.sectionId,
        sectionIdx: props.levelIdx,
        tag: 'discovery',
        idx: 0,
    })
    if (hasNext.value) goNext()
    else goNextSection()
}

// ── Problem-mode state ─────────────────────────────────────────────────
const problemRef = computed(() => problem.value)
const { goalLatex, premiseLatex, updateLatex } = useProblemLatex(problemRef)
const victory = useVictory()

const prefsOpen = ref(false)
const agreed = ref(false)
const showRepl = ref(false)

// ── Victory proof lines ────────────────────────────────────────────────
const sectionEntries = computed(() => {
    const bySection = roadmap.bySection
    return (bySection[props.sectionId] ?? [])
        .sort((a, b) => a.sectionIdx - b.sectionIdx)
})

const proofLines = computed(() => {
    const entry = sectionEntries.value.find(e => e.sectionIdx === props.levelIdx)
    if (!entry || entry.tag !== 'problem') return []
    return entry.proof.map((s, i) => `${i + 1}. ${renderNlg(s.cmdName, s.params, loadNlg(locale.value))}`)
})

const roadmapOpen = ref(false)
const showHelpGlobal = ref(false)
const helpStartTab = ref<'commands' | 'notations' | 'glossary'>('commands')

function openHelp(tab: 'commands' | 'notations' | 'glossary' = 'commands') {
    helpStartTab.value = tab
    showHelpGlobal.value = true
}

watch(
    () => [props.sectionId, props.levelIdx, isDiscovery.value],
    () => {
        agreed.value = false
        showRepl.value = false
        victory.solved.value = false
        discoveryIdx.value = 0
        updateLatex()
    },
)

async function onAgree() {
    agreed.value = true
    await nextTick()
    setTimeout(() => { showRepl.value = true }, 500)
}

function clearCurrent() {
    agreed.value = false
    showRepl.value = false
    victory.solved.value = false
    updateLatex()
    progress.clearCurrentProgress()
}

// ── Logic mode & allowed tactics ───────────────────────────────────────
const logicMode = computed(() =>
    problem.value?.logicMode ?? section.value?.meta.logicMode ?? 'fol',
)
const axiomSetLabel = computed(() => {
    if (!section.value) return ''
    const mode = logicMode.value
    if (mode === 'pl') return '(PL) FOL OFF'
    if (mode === 'fol') return '(FOL) FOL ON'
    return mode
})

const axiomSetTitle = computed(() => {
    if (logicMode.value === 'fol') return 'FOL Axioms'
    return 'PL Axioms'
})

const allowedTactics = computed(() => section.value?.meta.allowedTactics ?? [])
</script>

<template>
    <!-- Not found -->
    <div v-if="!section" class="not-found">
        {{ t('problem.notFound') }}
    </div>

    <!-- Discovery level -->
    <div v-else-if="isDiscovery" class="discovery-mode">
        <NavBar @open-prefs="prefsOpen = true" @open-help="openHelp('commands')">
            <span class="goal-chip">{{ sectionName }}</span>
        </NavBar>
        <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />
        <div v-if="discoveryIdx > 0" class="progress-track">
            <div class="progress-fill" :style="{ width: discoveryIdx / ((discoveryData?.lines.length ?? 1)) * 100 + '%' }"></div>
        </div>
        <div class="stage">
            <DiscoveryDialog v-if="discoveryData" :discovery="discoveryData" :section-id="props.sectionId" :level-idx="props.levelIdx"
                @progress="onDiscoveryProgress" @complete="onDiscoveryComplete" @skip="onDiscoverySkip" />
        </div>
        <div class="discovery-footer">
            <button class="nav-btn nav-btn-clear" @click="clearCurrent">{{ t('problem.clear') }}</button>
        </div>
        <HelpModal v-if="showHelpGlobal" @close="showHelpGlobal = false" :allowed-tactics="allowedTactics"
            :start-tab="helpStartTab" />
    </div>

    <!-- Problem level -->
    <div v-else-if="problem" class="root-row">
        <div class="root">
            <NavBar @open-prefs="prefsOpen = true" @open-help="openHelp('commands')">
                <span class="goal-chip"><span class="axiom-wrap" @click.stop="openHelp('notations')">
                        <span class="axiom-dot"
                            :class="{ 'axiom-dot--fol': logicMode === 'fol', 'axiom-dot--pl': logicMode === 'pl' }"></span>
                        <span class="axiom-chip"
                            :class="{ 'axiom-chip--fol-off': logicMode === 'pl' }">{{ sectionName }} · {{ axiomSetLabel
                            }}</span>
                        <span class="axiom-pop">
                            <span class="axiom-pop-title"
                                :class="{ 'axiom-pop-title--pl': logicMode === 'pl' }">{{ axiomSetTitle }}</span>
                            <span class="axiom-pop-row" v-if="logicMode === 'fol'"><span
                                    class="axiom-pop-mark axiom-pop-mark--on">✓</span> Quantifiers</span>
                            <span class="axiom-pop-row" v-else><span
                                    class="axiom-pop-mark axiom-pop-mark--off">✗</span> Quantifiers</span>
                            <span class="axiom-pop-row" v-if="logicMode === 'pl'"><span
                                    class="axiom-pop-mark axiom-pop-mark--on">✓</span> Modal</span>
                            <span class="axiom-pop-row" v-else><span
                                    class="axiom-pop-mark axiom-pop-mark--off">✗</span> Modal</span>
                        </span>
                    </span></span>
            </NavBar>

            <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />

            <div class="body">
                <button @click="goPrev" :disabled="!hasPrev" class="nav-btn nav-btn-top">{{ t('problem.prev') }}</button>

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
                        :logic-mode="logicMode" :allowed-tactics="allowedTactics" @stepTaken="onStepTaken" @solved="onSolved"
                        style="flex:1;overflow:hidden" />
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
                            <button v-if="hasNext" class="victory-btn" @click="goNext">
                                {{ t('problem.nextProblem') }}
                            </button>
                            <button v-else class="victory-btn" @click="goNextSection">
                                {{ t('sections.nextSection') }}
                            </button>
                        </div>
                    </div>
                </Transition>

                <button @click="goNext" :disabled="!hasNext" class="nav-btn nav-btn-bot">{{ t('problem.next') }}</button>
            </div>

            <div class="footer">
                <button @click="goPrev" :disabled="!hasPrev" class="nav-btn nav-btn-ft">{{ t('problem.prev') }}</button>
                <div class="footer-roadmap" @click="roadmapOpen = true">
                    <div class="mini-track">
                        <div v-for="(entry, i) in sectionEntries" :key="entry.sectionIdx" class="mini-dot"
                            :class="{ latest: i === sectionEntries.length - 1 }">
                            <span class="mini-dot-num">{{ entry.sectionIdx }}</span>
                        </div>
                    </div>
                </div>
                <button @click="goNext" :disabled="!hasNext" class="nav-btn nav-btn-ft">{{ t('problem.next') }}</button>
                <button class="nav-btn nav-btn-clear" @click="clearCurrent">{{ t('problem.clear') }}</button>
            </div>
        </div>

        <TacticSidebar :allowed-tactics="allowedTactics" />
        <HelpModal v-if="showHelpGlobal" @close="showHelpGlobal = false" :allowed-tactics="allowedTactics"
            :start-tab="helpStartTab" :axiom-flash-mode="logicMode" />
        <RoadmapModal v-if="roadmapOpen" :section-id="props.sectionId" @close="roadmapOpen = false" />
    </div>
</template>

<style lang="scss" scoped>
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

.goal-chip {
    font-size: 13px;
    color: var(--color-muted);
}

// ── Discovery mode ──────────────────────────────────────────────
.discovery-mode {
    display: flex;
    flex-direction: column;
    height: 100%;

    .progress-track {
        height: 3px;
        background: var(--color-border);
        border-radius: 0;
        overflow: hidden;
        flex-shrink: 0;

        .progress-fill {
            height: 100%;
            width: 0;
            background: var(--color-primary);
            border-radius: 0;
            transition: width 0.3s ease;
        }
    }

    .stage {
        flex: 1;
        overflow: hidden;
    }
}

.discovery-footer {
    display: flex;
    justify-content: flex-end;
    padding: 4px 12px;
    border-top: 1px solid var(--color-border);
    flex-shrink: 0;
}

// ── Axiom chip ──────────────────────────────────────────────────
.axiom-wrap {
    position: relative;
    cursor: pointer;

    &:hover .axiom-pop {
        opacity: 1;
    }
}

.axiom-chip {
    font-size: 11px;
    font-weight: 600;
    color: var(--color-fol-on);

    &--fol-off {
        color: var(--color-fol-off);
    }
}

.axiom-pop {
    position: absolute;
    top: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 6px 10px;
    border-radius: 6px;
    background: var(--color-fg);
    color: var(--color-bg);
    font-size: 11px;
    white-space: nowrap;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.15s;
    z-index: 100;

    &::before {
        content: '';
        position: absolute;
        bottom: 100%;
        left: 50%;
        transform: translateX(-50%);
        border: 4px solid transparent;
        border-bottom-color: var(--color-fg);
    }
}

.axiom-dot {
    display: none;
    width: 10px;
    height: 10px;
    border-radius: 100%;
    flex-shrink: 0;
    vertical-align: middle;
    margin-right: 6px;

    &--fol {
        background: var(--color-fol-on);
    }

    &--pl {
        background: var(--color-fol-off);
    }
}

.axiom-pop-title {
    font-weight: 600;
    font-size: 10px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--color-fol-on);
    margin-bottom: 1px;

    &--pl {
        color: var(--color-fol-off);
    }
}

.axiom-pop-row {
    font-size: 11px;
    line-height: 1.5;
}

.axiom-pop-mark {
    &--on {
        color: var(--color-axiom-check);
    }

    &--off {
        color: var(--color-axiom-x);
    }
}

.body {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
}

.not-found {
    padding: 32px;
}

// ── Footer ──────────────────────────────────────────────────────
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

    &:hover {
        opacity: 0.7;
    }
}

.nav-btn {
    padding: 4px 12px;
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
}

.nav-btn-clear {
    color: var(--color-muted);
    background: none;
    border: 1px solid transparent;
    font-size: 11px;
    padding: 2px 8px;

    &:hover {
        color: #c0392b;
        border-color: var(--color-border);
    }
}

// Flanking buttons — hidden on desktop, shown flanking the card on narrow
.nav-btn-top,
.nav-btn-bot {
    display: none;
    flex-shrink: 0;
}

@media (max-width: 500px) {

    .nav-btn-top,
    .nav-btn-bot {
        display: block;
        width: 100%;
        padding: 6px 12px;
    }

    .nav-btn-top {
        margin-bottom: 6px;
    }

    .nav-btn-bot {
        margin-top: 6px;
    }

    .nav-btn-ft {
        display: none;
    }

    .footer {
        justify-content: center;
        padding: 6px 8px;
    }
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

    &:nth-child(1) {
        animation-delay: 0s;
    }

    &:nth-child(2) {
        animation-delay: 0.05s;
    }

    &:nth-child(3) {
        animation-delay: 0.1s;
    }

    &:nth-child(4) {
        animation-delay: 0.15s;
    }

    &:nth-child(5) {
        animation-delay: 0.2s;
    }

    &:nth-child(6) {
        animation-delay: 0.25s;
    }

    &.latest {
        background: var(--color-primary);
        border-color: var(--color-primary);

        .mini-dot-num {
            color: var(--color-primary-fg);
        }
    }
}

.mini-dot-num {
    font-size: 10px;
    font-weight: 600;
    color: var(--color-muted);
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

// ── Prompt ──────────────────────────────────────────────────────
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

    &:hover {
        background: var(--color-primary-hover);
    }
}

// ── Victory ─────────────────────────────────────────────────────
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

    &:hover {
        background: var(--color-primary-hover);
    }
}

// ── Transitions ─────────────────────────────────────────────────
.fade-up-enter-active,
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

.fade-in-enter-from,
.fade-in-leave-to {
    opacity: 0;
}

// ── Responsive ──────────────────────────────────────────────────
@media (max-width: 600px) {
    .goal-chip {
        .axiom-chip {
            display: none;
        }

        .axiom-dot {
            display: inline-block;
        }

        .axiom-pop {
            left: -4px;
            transform: none;

            &::before {
                left: 8px;
            }
        }
    }

    .prompt {
        padding: 24px 12px;
    }

    .prove-desc {
        font-size: clamp(17px, 5vw, 24px);
    }

    .goal-line {
        flex-wrap: wrap;
        justify-content: center;
        gap: 6px;
    }

    .premise-katex {
        margin-right: 0;
    }

    .victory-text {
        font-size: clamp(22px, 7vw, 36px);
    }

    .proof-scroll {
        width: 94vw;
        padding: 12px;
        font-size: 11px;
    }

    .victory {
        padding: 12px 8px;
    }
}
</style>
