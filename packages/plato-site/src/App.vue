<script setup lang="ts">
import { ref, computed } from 'vue'
import { problems } from '@/data/problems'
import { useProgressStore } from '@/stores/progress'
import HomeView from '@/views/HomeView.vue'
import ProblemView from '@/views/ProblemView.vue'

const progress = useProgressStore()

type Page =
    | { type: 'home' }
    | { type: 'problem'; idx: number }

const page = ref<Page>({ type: 'home' })

const problemIdx = computed(() =>
    page.value.type === 'problem' ? page.value.idx : 0
)

/** Unique key that changes every time the page changes, for transition. */
const pageKey = computed(() =>
    page.value.type === 'home' ? 'home' : `problem-${page.value.idx}`
)

function goHome() {
    page.value = { type: 'home' }
}

function onStart() {
    progress.highestSolved = 0;
    page.value = { type: 'problem', idx: 0 }
}

function onContinue() {
    const idx = Math.min(progress.highestSolved + 1, problems.length - 1)
    page.value = { type: 'problem', idx }
}

function onNext() {
    if (page.value.type === 'problem') {
        const next = page.value.idx + 1
        if (next < problems.length) {
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
              <HomeView
                v-if="page.type === 'home'"
                :key="pageKey"
                :has-progress="progress.highestSolved >= 0"
                @start="onStart"
                @continue="onContinue"
              />
              <ProblemView
                v-else
                :key="pageKey"
                :problem-idx="problemIdx"
                :problems="problems"
                @next="onNext"
                @prev="onPrev"
                @home="goHome"
              />
            </Transition>
        </div>
        <footer class="footer">
            All Rights Reserved
            <a href="https://blog.makabaka1880.xyz" target="_blank">Makabaka1880</a>
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
