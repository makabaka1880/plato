<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { loadSections } from '@/data'
import { useProgressStore } from '@/stores/progress'
import { useDiscoveryStore } from '@/stores/discovery'
import { useTacticsStore } from '@/stores/tactics'
import { useRoadmapStore } from '@/stores/roadmap'
import { usePreferencesStore } from '@/stores/preferences'

const router = useRouter()
const { t, locale } = useI18n()
const prefs = usePreferencesStore()
const progress = useProgressStore()
const discovery = useDiscoveryStore()

function setLocale(loc: string) {
    locale.value = loc
    prefs.setLocale(loc)
}

// ── Section data ────────────────────────────────────────────────────
const allSections = computed(() => loadSections(locale.value))

const visibleSections = computed(() =>
    allSections.value.filter(s => progress.isSectionAccessible(s.id, allSections.value)),
)

const hasProgress = computed(() => {
    for (const sec of allSections.value) {
        if ((progress.highestSolved[sec.id] ?? -1) >= 0) return true
    }
    return false
})

// ── Continue target ─────────────────────────────────────────────────
const continueTarget = computed(() =>
    progress.continueTarget(allSections.value),
)

function goContinue() {
    const tgt = continueTarget.value
    if (!tgt) return
    if (!discovery.isViewed(tgt.sectionId) && tgt.sectionIdx === 0) {
        router.push(`/section/${tgt.sectionId}/discovery`)
    } else {
        router.push(`/section/${tgt.sectionId}/problem/${tgt.sectionIdx}`)
    }
}

function onStartFresh() {
    progress.reset()
    discovery.reset()
    useTacticsStore().reset()
    useRoadmapStore().reset()
    const sorted = [...allSections.value].sort((a, b) => a.meta.order - b.meta.order)
    if (sorted.length > 0) {
        router.push(`/section/${sorted[0]!.id}/discovery`)
    }
}

// ── Section card interaction ────────────────────────────────────────
function onSectionClick(sectionId: string) {
    const sec = allSections.value.find(s => s.id === sectionId)
    if (!sec) return
    const highest = progress.highestSolved[sectionId] ?? -1
    const nextIdx = highest + 1
    if (nextIdx < sec.problems.length) {
        if (!discovery.isViewed(sectionId) && nextIdx === 0) {
            router.push(`/section/${sectionId}/discovery`)
        } else {
            router.push(`/section/${sectionId}/problem/${nextIdx}`)
        }
    }
}

function replayDiscovery(sectionId: string) {
    router.push(`/section/${sectionId}/discovery`)
}

function isSectionComplete(sectionId: string): boolean {
    const sec = allSections.value.find(s => s.id === sectionId)
    if (!sec) return false
    const highest = progress.highestSolved[sectionId] ?? -1
    return highest + 1 >= sec.problems.length
}

// ── Focus ───────────────────────────────────────────────────────────
const continueBtn = ref<HTMLButtonElement | null>(null)
const startBtn = ref<HTMLButtonElement | null>(null)

onMounted(() => {
    if (hasProgress.value) continueBtn.value?.focus()
    else startBtn.value?.focus()
})
</script>

<template>
    <div class="hero">
        <div class="lang-switch">
            <button :class="{ active: locale === 'en' }" @click="setLocale('en')">EN</button>
            <button :class="{ active: locale === 'zh' }" @click="setLocale('zh')">中文</button>
        </div>
        <h1>{{ t('home.title') }}</h1>

        <div class="actions">
            <button
                v-if="hasProgress"
                ref="continueBtn"
                class="hero-btn"
                @click="goContinue"
            >
                {{ t('home.continue') }}
            </button>
            <button
                ref="startBtn"
                class="hero-btn"
                :class="hasProgress ? 'secondary' : ''"
                @click="onStartFresh"
            >
                {{ t('home.startFresh') }}
            </button>
            <button class="hero-btn custom-btn" @click="router.push('/custom')">
                {{ t('home.customProblem') }}
            </button>
        </div>

        <!-- Section cards -->
        <div v-if="visibleSections.length > 0" class="sections">
            <div
                v-for="section in visibleSections"
                :key="section.id"
                class="section-card"
                :class="{ complete: isSectionComplete(section.id) }"
                @click="onSectionClick(section.id)"
            >
                <div class="section-name">{{ t(section.meta.nameI18nKey) }}</div>
                <div class="section-progress">
                    <span
                        v-for="i in section.problems.length"
                        :key="i"
                        class="section-dot"
                        :class="{ solved: (progress.highestSolved[section.id] ?? -1) >= i - 1 }"
                    ></span>
                </div>
                <div class="section-meta">
                    <span class="section-count">{{ t('sections.solved', {
                        solved: Math.min((progress.highestSolved[section.id] ?? -1) + 1, section.problems.length),
                        total: section.problems.length
                    }) }}</span>
                    <button
                        v-if="discovery.isViewed(section.id)"
                        class="replay-btn"
                        @click.stop="replayDiscovery(section.id)"
                    >
                        {{ t('discovery.replay') }}
                    </button>
                </div>
            </div>
        </div>

        <div class="sub-actions">
            <a href="https://github.com/makabaka1880/plato">{{ t('home.contribute') }}</a>
            <span class="sub-sep">·</span>
            <router-link to="/about">{{ t('home.about') }}</router-link>
            <span class="sub-sep">·</span>
            <a href="https://blog.makabaka1880.xyz/articles/260625-building-plato">{{ t('home.story') }}</a>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.hero {
  display: flex; flex-direction: column; align-items: center;
  justify-content: flex-start; height: 100%; padding-top: 6vh; overflow-y: auto;
  h1 { font-size: clamp(28px, 6vw, 56px); font-weight: 400; letter-spacing: -0.02em; margin-bottom: 32px; }
}

.lang-switch {
  margin-bottom: 20px; display: flex; gap: 2px;
  button {
    font-family: inherit; font-size: 11px; font-weight: 600; letter-spacing: 0.04em;
    color: var(--color-muted); background: none; border: 1px solid transparent;
    border-radius: 3px; padding: 3px 8px; cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
    &:hover { color: var(--color-fg); border-color: var(--color-border); }
    &.active { color: var(--color-fg); border-color: var(--color-border); background: var(--color-subtle-bg); }
  }
}

.actions { display: flex; flex-direction: column; gap: 8px; align-items: center; margin-bottom: 32px; }

.hero-btn {
  font-family: inherit; font-size: 15px; padding: 8px 0; cursor: pointer;
  width: 200px; background: var(--color-primary); color: var(--color-primary-fg);
  border: none; border-radius: 4px;
  &:hover { background: var(--color-primary-hover); }
  &.secondary, &.custom-btn {
    background: transparent; border: 1px solid var(--color-border); color: var(--color-muted);
    &:hover { background: var(--color-subtle-bg); border-color: var(--color-primary-hover); color: var(--color-primary-hover); }
  }
}

// ── Section cards ─────────────────────────────────────────────
.sections { display: flex; flex-direction: column; gap: 8px; width: min(320px, 80vw); margin-bottom: 16px; }

.section-card {
  padding: 12px 16px; border: 1px solid var(--color-border);
  border-radius: 6px; cursor: pointer; transition: border-color 0.15s, background 0.15s;
  &:hover { border-color: var(--color-primary-hover); background: var(--color-subtle-bg); }
  &.complete { opacity: 0.7; }
}
.section-name { font-size: 14px; font-weight: 600; color: var(--color-fg); margin-bottom: 8px; }
.section-progress { display: flex; gap: 4px; margin-bottom: 8px; }
.section-dot {
  width: 8px; height: 8px; border-radius: 100%; background: var(--color-border); transition: background 0.3s;
  &.solved { background: var(--color-primary); }
}
.section-meta { display: flex; align-items: center; justify-content: space-between; }
.section-count { font-size: 11px; color: var(--color-muted); }
.replay-btn {
  font-family: inherit; font-size: 11px; padding: 2px 8px; cursor: pointer;
  background: none; border: 1px solid var(--color-border); border-radius: 3px;
  color: var(--color-muted); transition: color 0.15s, border-color 0.15s;
  &:hover { color: var(--color-primary-hover); border-color: var(--color-primary-hover); }
}

.sub-actions {
  display: flex; gap: 0; align-items: center; justify-content: center;
  margin-top: 16px; font-size: 12px; color: var(--color-border-strong);
  a {
    color: inherit; text-decoration: none; padding: 2px 6px; cursor: pointer; transition: color 0.15s;
    &:hover { color: var(--color-muted); }
  }
  .sub-sep { color: var(--color-border); user-select: none; }
}
</style>
