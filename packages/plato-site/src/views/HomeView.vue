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

const allSections = computed(() => loadSections(locale.value))

const visibleSections = computed(() => allSections.value)

const isSectionAccessible = (sectionId: string) => progress.isSectionAccessible(sectionId, allSections.value)

const hasProgress = computed(() => {
    for (const sec of allSections.value) {
        if ((progress.highestSolved[sec.id] ?? -1) >= 0) return true
    }
    return false
})

const allComplete = computed(() => visibleSections.value.length > 0 &&
    visibleSections.value.every(s => {
        if (!isSectionAccessible(s.id)) return false
        const highest = progress.highestSolved[s.id] ?? -1
        return highest + 1 >= s.problems.length
    })
)

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

function onSectionClick(sectionId: string) {
    router.push(`/section/${sectionId}/problem/0`)
}

function replayDiscovery(sectionId: string) {
    router.push(`/section/${sectionId}/discovery`)
}

function sectionProgressPercent(sectionId: string): number {
    const sec = allSections.value.find(s => s.id === sectionId)
    if (!sec || sec.problems.length === 0) return 0
    const solved = (progress.highestSolved[sectionId] ?? -1) + 1
    return Math.min(100, (solved / sec.problems.length) * 100)
}

const continueBtn = ref<HTMLButtonElement | null>(null)
const startBtn = ref<HTMLButtonElement | null>(null)

const copied = ref(false)

async function copyEmail() {
    try {
        await navigator.clipboard.writeText('makabaka1880@outlook.com')
        copied.value = true
        setTimeout(() => { copied.value = false }, 1800)
    } catch {
        // fallback silently
    }
}

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
            <button v-if="!hasProgress" ref="startBtn" class="hero-btn" @click="onStartFresh">
                {{ t('home.begin') }}
            </button>
            <button v-if="hasProgress && !allComplete" ref="continueBtn" class="hero-btn" @click="goContinue">
                {{ t('home.continue') }}
            </button>
            <button v-if="hasProgress" class="hero-btn secondary" @click="onStartFresh">
                {{ t('home.startFresh') }}
            </button>
            <button class="hero-btn custom-btn" @click="router.push('/custom')">
                {{ t('home.customProblem') }}
            </button>
        </div>

        <div v-if="allComplete" class="all-done">{{ t("home.allDone") }}</div>

        <div v-if="visibleSections.length > 0" class="sections">
            <div v-for="section in visibleSections" :key="section.id" class="section-card"
                :class="{ locked: !isSectionAccessible(section.id) }"
                @click="isSectionAccessible(section.id) && onSectionClick(section.id)">
                <template v-if="isSectionAccessible(section.id)">
                    <div class="section-head">
                        <div class="section-name">{{ t(section.meta.nameI18nKey) }}</div>
                        <button class="replay-btn" @click.stop="replayDiscovery(section.id)">
                            {{ discovery.isViewed(section.id) ? t('discovery.replay') : t('discovery.play') }}
                        </button>
                    </div>
                    <div class="progress-track">
                        <div class="progress-fill" :style="{ width: sectionProgressPercent(section.id) + '%' }"></div>
                    </div>
                    <div class="section-count">
                        {{ (progress.highestSolved[section.id] ?? -1) + 1 }} / {{ section.problems.length }}
                    </div>
                </template>
                <template v-else>
                    <div class="section-head">
                        <div class="section-name locked-name">???</div>
                    </div>
                    <div class="progress-track locked-track"></div>
                    <div class="section-count locked-count">{{ t('locked.title') }}</div>
                </template>
            </div>
        </div>

        <div class="sub-actions">
            <a href="https://github.com/makabaka1880/plato">{{ t('home.contribute') }}</a>
            <span class="sub-sep">·</span>
            <router-link to="/about">{{ t('home.about') }}</router-link>
            <span class="sub-sep">·</span>
            <a href="https://blog.makabaka1880.xyz/articles/260625-building-plato">{{ t('home.story') }}</a>
            <span class="sub-sep">·</span>
            <span class="contact-wrap">
                <span class="contact-trigger" @click="copyEmail">{{ t('home.contact') }}</span>
                <span class="contact-pop" :class="{ copied }">{{ copied ? t('common.copied') : 'makabaka1880@outlook.com' }}</span>
            </span>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    overflow-y: auto;

    h1 {
        font-size: clamp(28px, 6vw, 56px);
        font-weight: 400;
        letter-spacing: -0.02em;
        margin-bottom: 28px;
    }
}

.lang-switch {
    margin-bottom: 20px;
    display: flex;
    gap: 2px;

    button {
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

        &:hover { color: var(--color-fg); border-color: var(--color-border); }
        &.active { color: var(--color-fg); border-color: var(--color-border); background: var(--color-subtle-bg); }
    }
}

.actions {
    display: flex; flex-direction: column; gap: 8px;
    align-items: center; margin-bottom: 28px;
}

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

.all-done {
    font-size: 14px; color: var(--color-primary-hover);
    font-weight: 500; text-align: center;
    margin-bottom: 20px; opacity: 0.85;
}

// ── Section cards ─────────────────────────────────────────────
.sections {
    display: flex; flex-direction: column; gap: 6px;
    width: min(400px, 80vw); margin-bottom: 20px;
}

.section-card {
    padding: 10px 16px;
    border: 1px solid var(--color-border);
    border-radius: 6px; cursor: pointer;
    transition: border-color 0.15s, background 0.15s;

    &:hover:not(.locked) { border-color: var(--color-primary-hover); background: var(--color-subtle-bg); }
    &.locked { cursor: default; opacity: 0.35; }
}

.section-head {
    display: flex; align-items: center; justify-content: space-between;
    margin-bottom: 8px;
}

.section-name {
    font-size: 13px; font-weight: 600; color: var(--color-fg);
}

.replay-btn {
    font-family: inherit; font-size: 10px; padding: 2px 8px; cursor: pointer;
    background: none; border: 1px solid var(--color-border); border-radius: 3px;
    color: var(--color-muted);
    transition: color 0.15s, border-color 0.15s;

    &:hover { color: var(--color-primary-hover); border-color: var(--color-primary-hover); }
}

.progress-track {
    height: 3px; background: var(--color-border); border-radius: 2px;
    overflow: hidden; margin-bottom: 6px;
}

.progress-fill {
    height: 100%; width: 0; background: var(--color-primary);
    border-radius: 2px; transition: width 0.5s ease;
}

.section-count {
    font-size: 10px; color: var(--color-muted); text-align: right;
}
.locked-name { color: var(--color-border-strong) !important; }
.locked-track { background: var(--color-border) !important; opacity: 0.3; }
.locked-count { text-align: right; opacity: 0.5; }

.sub-actions {
    display: flex; gap: 0; align-items: center; justify-content: center;
    margin-top: 8px; font-size: 12px; color: var(--color-border-strong);

    a {
        color: var(--color-subtle); text-decoration: none; padding: 2px 6px; cursor: pointer;
        transition: color 0.15s, border-color 0.15s;
        border-bottom: 1px solid transparent;
        &:hover { color: var(--color-primary); border-bottom-color: var(--color-primary-hover); }
    }

    .sub-sep { color: var(--color-border); user-select: none; }

    .contact-wrap { position: relative; }
    .contact-trigger {
        color: var(--color-subtle); padding: 2px 6px; cursor: pointer;
        transition: color 0.15s, border-color 0.15s;
        border-bottom: 1px solid transparent;
        &:hover { color: var(--color-primary); border-bottom-color: var(--color-primary-hover); }
    }
    .contact-pop {
        position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%);
        margin-bottom: 6px; padding: 4px 10px; border-radius: 4px;
        background: var(--color-fg); color: var(--color-bg);
        font-size: 11px; white-space: nowrap; opacity: 0; pointer-events: none;
        transition: opacity 0.15s;
        &::after {
            content: ''; position: absolute; top: 100%; left: 50%; transform: translateX(-50%);
            border: 4px solid transparent; border-top-color: var(--color-fg);
        }
        &.copied {
            opacity: 1; background: var(--color-primary); color: var(--color-primary-fg);
            &::after { border-top-color: var(--color-primary); }
        }
    }
    .contact-wrap:hover .contact-pop:not(.copied) { opacity: 1; }
}
</style>
