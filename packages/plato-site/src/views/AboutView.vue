<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { marked } from 'marked'
import katex from 'katex'
import NavBar from '@/components/NavBar.vue'
import PreferenceModal from '@/components/PreferenceModal.vue'
import HelpModal from '@/components/HelpModal.vue'

const { locale } = useI18n()

const prefsOpen = ref(false)
const showHelp = ref(false)

// Lazy-load ABOUT.md for the current locale
const aboutModules = import.meta.glob(
    '../data/*/ABOUT.md',
    { query: '?raw', eager: true },
) as Record<string, { default: string }>

function loadContent(loc: string): string {
    const key = `../data/${loc}/ABOUT.md`
    if (aboutModules[key]) return aboutModules[key]!.default
    return aboutModules['../data/en/ABOUT.md']!.default
}

function renderTex(html: string): string {
    // Render $$...$$ (display math), then $...$ (inline math)
    return html
        .replace(/\$\$([^$]+)\$\$/g, (_, expr) => {
            try {
                return katex.renderToString(expr.trim(), { displayMode: true, throwOnError: false })
            } catch {
                return `<code>${expr}</code>`
            }
        })
        .replace(/\$([^$]+)\$/g, (_, expr) => {
            try {
                return katex.renderToString(expr.trim(), { displayMode: false, throwOnError: false })
            } catch {
                return `<code>${expr}</code>`
            }
        })
}

function parse(md: string): string {
    const raw = marked.parse(md, { async: false }) as string
    return renderTex(raw)
}

const html = ref(parse(loadContent(locale.value)))

watch(locale, (loc) => {
    html.value = parse(loadContent(loc))
})

// ── Scroll progress ──────────────────────────────────────────────────
const scrollEl = ref<HTMLElement | null>(null)
const progress = ref(0)

function onScroll() {
    const el = scrollEl.value
    if (!el) return
    const h = el.scrollHeight - el.clientHeight
    progress.value = h > 0 ? el.scrollTop / h : 0
}

onMounted(() => {
    scrollEl.value?.addEventListener('scroll', onScroll, { passive: true })
})
onUnmounted(() => {
    scrollEl.value?.removeEventListener('scroll', onScroll)
})
</script>

<template>
    <div class="about-root">
        <NavBar @open-prefs="prefsOpen = true" @open-help="showHelp = true" />
        <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${progress * 100}%` }"></div>
        </div>
        <!-- eslint-disable-next-line vue/no-v-html -->
        <div ref="scrollEl" class="scroll-container">
            <div class="content" v-html="html"></div>
        </div>
        <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />
        <HelpModal v-if="showHelp" @close="showHelp = false" />
    </div>
</template>

<style lang="scss" scoped>
.about-root { display: flex; flex-direction: column; height: 100%; overflow: hidden; }
.progress-bar { height: 2px; background: var(--color-border); flex-shrink: 0; }
.progress-fill { height: 100%; width: 0; background: var(--color-primary-hover); transition: width 0.05s linear; }

.scroll-container {
  flex: 1; overflow-y: auto; scrollbar-width: none;
  &::-webkit-scrollbar { display: none; }
}

.content {
  max-width: 640px; margin: 0 auto; padding: 32px 20px 48px;
  box-sizing: border-box; font-size: 14px; line-height: 1.75; color: var(--color-fg);

  :deep {
    h1 { font-size: 28px; font-weight: 400; letter-spacing: -0.02em; margin: 0 0 24px; }
    h2 { font-size: 18px; font-weight: 600; margin: 40px 0 12px; padding-bottom: 6px; border-bottom: 1px solid var(--color-border); }
    h3 { font-size: 15px; font-weight: 600; margin: 24px 0 8px; }
    p { margin: 0 0 14px; }
    ul { margin: 0 0 14px; padding-left: 20px; }
    li { margin-bottom: 4px; }
    code { font-family: inherit; font-size: 0.92em; background: var(--color-subtle-bg); padding: 1px 5px; border-radius: 3px; }
    pre {
      background: var(--color-subtle-bg); border: 1px solid var(--color-border);
      border-radius: 6px; padding: 12px 16px; overflow-x: auto;
      margin: 0 0 14px; font-size: 13px; line-height: 1.6;
      code { background: none; padding: 0; border-radius: 0; }
    }
    blockquote {
      border-left: 3px solid var(--color-border); margin: 0 0 14px;
      padding: 4px 0 4px 14px; color: var(--color-muted);
      p { margin-bottom: 4px; }
    }
    hr { border: none; border-top: 1px solid var(--color-border); margin: 24px 0; }
    table { width: 100%; border-collapse: collapse; margin: 0 0 14px; font-size: 13px; }
    th, td { text-align: left; padding: 6px 12px; border-bottom: 1px solid var(--color-border); }
    th { font-weight: 600; color: var(--color-muted); }
    a { color: var(--color-primary-hover); text-decoration: none;
      &:hover { text-decoration: underline; }
    }
  }
}
</style>
