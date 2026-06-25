<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoadmapStore, type RoadmapEntry } from '@/stores/roadmap'
import { loadSections } from '@/data'
import { useProgressStore } from '@/stores/progress'
import InlineLatex from './InlineLatex.vue'

const props = defineProps<{ sectionId?: string }>()

const emit = defineEmits<{ close: [] }>()

const { t, locale } = useI18n()
const store = useRoadmapStore()
const progress = useProgressStore()
const selected = ref<RoadmapEntry | null>(null)

// If sectionId provided, show only that section's entries
// Otherwise show all, grouped by section
const sections = computed(() => loadSections(locale.value))

const visibleSections = computed(() => {
    if (props.sectionId) {
        return sections.value.filter(s => s.id === props.sectionId)
    }
    // Only show accessible sections
    return sections.value.filter(s => progress.isSectionAccessible(s.id, sections.value))
})

const bySection = computed(() => store.bySection)

function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') { e.preventDefault(); emit('close') }
}

function sectionName(sectionId: string): string {
    const sec = sections.value.find(s => s.id === sectionId)
    if (!sec) return sectionId
    return t(sec.meta.nameI18nKey)
}

onMounted(() => document.addEventListener('keydown', onKeydown))
onUnmounted(() => document.removeEventListener('keydown', onKeydown))
</script>

<template>
    <div class="rm-backdrop" @click.self="emit('close')">
        <div class="rm-modal">
            <div class="rm-head">
                <span class="rm-title">{{ t('roadmap.title') }}</span>
                <button class="rm-close" @click="emit('close')">&times;</button>
            </div>
            <div class="rm-body">
                <div v-if="store.entries.length === 0" class="rm-empty">
                    {{ t('roadmap.empty') }}
                </div>
                <template v-else v-for="section in visibleSections" :key="section.id">
                    <div class="rm-section-header">{{ sectionName(section.id) }}</div>
                    <div class="rm-list">
                        <div
                            v-for="entry in (bySection[section.id] ?? []).sort((a, b) => a.sectionIdx - b.sectionIdx)"
                            :key="entry.sectionIdx"
                            class="rm-stop"
                            :class="{ active: selected?.sectionId === entry.sectionId && selected?.sectionIdx === entry.sectionIdx }"
                        >
                            <button class="rm-stop-btn" @click="selected = (selected?.sectionId === entry.sectionId && selected?.sectionIdx === entry.sectionIdx) ? null : entry">
                                <span class="rm-stop-num">{{ entry.sectionIdx + 1 }}</span>
                                <span class="rm-stop-desc">{{ entry.description }}</span>
                            </button>
                            <div v-if="selected?.sectionId === entry.sectionId && selected?.sectionIdx === entry.sectionIdx" class="rm-proof">
                                <div class="rm-proof-label">{{ t('roadmap.proof') }}</div>
                                <div
                                    v-for="(line, j) in selected.proofLines"
                                    :key="j"
                                    class="rm-proof-line"
                                >
                                    <InlineLatex :text="line" />
                                </div>
                            </div>
                        </div>
                    </div>
                </template>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.rm-backdrop {
  position: fixed; inset: 0; z-index: 400;
  background: rgba(0,0,0,0.3); display: flex; align-items: center; justify-content: center;
}
.rm-modal {
  background: var(--color-bg); border-radius: 12px;
  max-width: 600px; width: 92vw; max-height: 80vh;
  display: flex; flex-direction: column; box-shadow: 0 8px 32px rgba(0,0,0,0.15);
}
.rm-head { display: flex; align-items: center; justify-content: space-between; padding: 16px 20px 12px; border-bottom: 1px solid var(--color-border); }
.rm-title { font-size: 13px; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; }
.rm-close { background: none; border: none; font-size: 22px; cursor: pointer; color: var(--color-muted); line-height: 1;
  &:hover { color: var(--color-fg); }
}
.rm-body { overflow-y: auto; padding: 14px 20px 20px; }
.rm-empty { font-size: 13px; color: var(--color-muted); text-align: center; padding: 40px 0; }
.rm-section-header {
  font-size: 11px; font-weight: 600; letter-spacing: 0.08em; color: var(--color-muted);
  text-transform: uppercase; margin: 12px 0 6px; padding-top: 4px;
  border-top: 1px solid var(--color-border);
  &:first-child { margin-top: 0; border-top: none; padding-top: 0; }
}
.rm-list { display: flex; flex-direction: column; gap: 8px; }
.rm-stop {
  border: 1px solid var(--color-border); border-radius: 8px; overflow: hidden;
  &.active { border-color: var(--color-primary-hover); }
}
.rm-stop-btn {
  display: flex; align-items: center; gap: 12px; width: 100%; text-align: left;
  font-family: inherit; font-size: 13px; padding: 10px 14px; cursor: pointer;
  background: var(--color-subtle-bg); border: none; color: var(--color-fg);
  transition: background 0.15s;
  &:hover { background: var(--color-border); }
}
.rm-stop-num {
  flex-shrink: 0; width: 28px; height: 28px; border-radius: 100%;
  background: var(--color-primary); color: var(--color-primary-fg);
  display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 600;
}
.rm-stop-desc { flex: 1; line-height: 1.45; }
.rm-proof { border-top: 1px solid var(--color-border); padding: 12px 14px 14px; background: var(--color-bg); }
.rm-proof-label { font-size: 10px; font-weight: 600; letter-spacing: 0.08em; color: var(--color-muted); text-transform: uppercase; margin-bottom: 8px; }
.rm-proof-line { font-size: 12px; line-height: 1.9; color: var(--color-subtle); }
</style>
