<script setup lang="ts">
import { ref, computed } from 'vue'
import { problems } from '@/data/problems'
import HomeView from '@/views/HomeView.vue'
import ProblemView from '@/views/ProblemView.vue'

type Page =
  | { type: 'home' }
  | { type: 'problem'; idx: number }

const page = ref<Page>({ type: 'home' })

const problemIdx = computed(() =>
  page.value.type === 'problem' ? page.value.idx : 0
)

function goHome() {
  page.value = { type: 'home' }
}

function onStart() {
  page.value = { type: 'problem', idx: 0 }
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
      <HomeView v-if="page.type === 'home'" @start="onStart" />
      <ProblemView
        v-else
        :problem-idx="problemIdx"
        :problems="problems"
        @next="onNext"
        @prev="onPrev"
        @home="goHome"
      />
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
