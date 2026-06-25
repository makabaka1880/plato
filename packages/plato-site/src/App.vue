<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { loadProblems } from '@/data'
import { useProgressStore } from '@/stores/progress'
import { useTacticsStore } from '@/stores/tactics'
import HomeView from '@/views/HomeView.vue'
import ProblemView from '@/views/ProblemView.vue'

const { t, locale } = useI18n()

const progress = useProgressStore()
const tactics = useTacticsStore()

type Page =
    | { type: 'home' }
    | { type: 'problem'; idx: number }

const page = ref<Page>({ type: 'home' })

const problems = computed(() => loadProblems(locale.value))

const problemIdx = computed(() =>
    page.value.type === 'problem' ? page.value.idx : 0
)

function goHome() {
    page.value = { type: 'home' }
}

function onStart() {
    progress.reset()
    tactics.reset()
    page.value = { type: 'problem', idx: 0 }
}

function onContinue() {
    const idx = Math.min(progress.highestSolved + 1, problems.value.length - 1)
    page.value = { type: 'problem', idx }
}

function onNext() {
    if (page.value.type === 'problem') {
        const next = page.value.idx + 1
        if (next < problems.value.length) {
            page.value = { type: 'problem', idx: next }
        }
    }
}

function onPrev() {
    if (page.value.type === 'problem' && page.value.idx > 0) {
        page.value = { type: 'problem', idx: page.value.idx - 1 }
    }
}
</script>

<template>
    <div class="app">
        <div class="main">
            <Transition name="page" mode="out-in">
                <HomeView v-if="page.type === 'home'" :has-progress="progress.highestSolved >= 0"
                    @start="onStart" @continue="onContinue" />
                <ProblemView v-else :problem-idx="problemIdx" :problems="problems" @next="onNext"
                    @prev="onPrev" @home="goHome" />
            </Transition>
        </div>
        <footer class="footer">
            {{ t('footer.rights') }}
            <a href="https://blog.makabaka1880.xyz" target="_blank">{{ t('footer.author') }}</a>
            &copy; 2026
        </footer>
    </div>
</template>

<style scoped>
.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

.main {
    flex: 1;
    overflow: hidden;
    position: relative;
}

/* ── page transitions ──────────────────── */
.page-enter-active,
.page-leave-active {
    transition: opacity 0.25s ease, transform 0.25s ease;
}

.page-enter-from {
    opacity: 0;
    transform: translateY(12px);
}

.page-leave-to {
    opacity: 0;
    transform: translateY(-8px);
}

.footer {
    flex-shrink: 0;
    text-align: center;
    padding: 6px 12px;
    font-size: 11px;
    color: var(--color-muted);
    border-top: 1px solid var(--color-border);
}

.footer a {
    color: inherit;
    font-weight: 500;
}
</style>
