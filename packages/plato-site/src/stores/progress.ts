import { defineStore } from 'pinia'
import { resolveGlobalIndex } from '@/data'
import type { Section } from '@/types'
import type { StepMeta } from '@/composables/useNlg'

const OLD_KEY = 'plato-highest'
const V2_KEY = 'plato-progress-v2'
const KEY = 'plato-progress-v3'

export type CurrentProgress =
    | { sectionId: string; levelIdx: number; tag: 'problem'; proof: StepMeta[] }
    | { sectionId: string; levelIdx: number; tag: 'discovery'; idx: number }

interface ProgressState {
    highestCompletedLevel: Record<string, number>
    currentSection: string | null
    currentProgress: CurrentProgress | null
}

function migrateOldGlobal(): ProgressState | null {
    try {
        const raw = localStorage.getItem(OLD_KEY)
        if (raw === null) return null
        const oldHighest = JSON.parse(raw) as number
        if (oldHighest < 0) return null

        // Map old global index to per-section progress.
        // resolveGlobalIndex returns 0-based problem indices; add +1 for level index
        // (discovery is level 0).
        const result: Record<string, number> = {}
        for (let globalIdx = 0; globalIdx <= oldHighest; globalIdx++) {
            const resolved = resolveGlobalIndex(globalIdx)
            if (resolved) {
                const levelIdx = resolved.sectionIdx + 1
                const cur = result[resolved.sectionId] ?? -1
                if (levelIdx > cur) {
                    result[resolved.sectionId] = levelIdx
                }
            }
        }
        const last = resolveGlobalIndex(oldHighest)

        localStorage.removeItem(OLD_KEY)
        return { highestCompletedLevel: result, currentSection: last?.sectionId ?? null, currentProgress: null }
    } catch { /* ignore */ }
    return null
}

/** Migrate from v2 (problem-index-based) to v3 (level-index-based). */
function migrateV2(): ProgressState | null {
    try {
        const raw = localStorage.getItem(V2_KEY)
        if (raw === null) return null
        const parsed = JSON.parse(raw)
        if (parsed && typeof parsed.highestSolved === 'object') {
            const old: Record<string, number> = parsed.highestSolved
            const result: Record<string, number> = {}
            for (const [sectionId, highest] of Object.entries(old)) {
                if (typeof highest === 'number' && highest >= 0) {
                    // Shift: old problem index N → new level index N+1 (discovery at level 0)
                    result[sectionId] = highest + 1
                }
            }
            localStorage.removeItem(V2_KEY)
            return { highestCompletedLevel: result, currentSection: parsed.currentSection ?? null, currentProgress: null }
        }
    } catch { /* ignore */ }
    return null
}

function load(): ProgressState {
    // Try v3 first
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (parsed && typeof parsed.highestCompletedLevel === 'object') {
                return {
                    highestCompletedLevel: parsed.highestCompletedLevel as Record<string, number>,
                    currentSection: (parsed.currentSection ?? null) as string | null,
                    currentProgress: (parsed.currentProgress ?? null) as CurrentProgress | null,
                }
            }
        }
    } catch { /* ignore */ }

    // Try migration from v2
    const v2 = migrateV2()
    if (v2) return v2

    // Try migration from old global format
    const global = migrateOldGlobal()
    if (global) return global

    return { highestCompletedLevel: {}, currentSection: null, currentProgress: null }
}

function save(state: ProgressState) {
    localStorage.setItem(KEY, JSON.stringify(state))
}

export const useProgressStore = defineStore('progress', {
    state: (): ProgressState => load(),

    actions: {
        markCompleted(sectionId: string, levelIdx: number) {
            const cur = this.highestCompletedLevel[sectionId] ?? -1
            if (levelIdx > cur) {
                this.highestCompletedLevel[sectionId] = levelIdx
            }
            // Clear current progress on completion
            if (this.currentProgress && this.currentProgress.sectionId === sectionId && this.currentProgress.levelIdx === levelIdx) {
                this.currentProgress = null
            }
            save(this.$state)
        },

        setCurrentSection(sectionId: string) {
            this.currentSection = sectionId
            save(this.$state)
        },

        /** Save in-progress state within a level. */
        saveCurrentProgress(cp: CurrentProgress) {
            this.currentProgress = cp
            save(this.$state)
        },

        /** Wipe in-progress state (Clear button). */
        clearCurrentProgress() {
            this.currentProgress = null
            save(this.$state)
        },

        /** Check if a level within a section is unlocked. Level 0 is always unlocked. */
        isUnlocked(sectionId: string, levelIdx: number): boolean {
            if (levelIdx === 0) return true
            return levelIdx <= (this.highestCompletedLevel[sectionId] ?? -1) + 1
        },

        /**
         * Check if a section is accessible. A section is accessible if:
         * - It's the first section (lowest order), OR
         * - All levels in all prior sections (by order) are completed
         */
        isSectionAccessible(sectionId: string, allSections: Section[]): boolean {
            const sorted = [...allSections].sort((a, b) => a.meta.order - b.meta.order)
            const targetIdx = sorted.findIndex(s => s.id === sectionId)
            if (targetIdx < 0) return false
            if (targetIdx === 0) return true

            // Check that all prior sections are fully completed
            for (let i = 0; i < targetIdx; i++) {
                const sec = sorted[i]!
                const highest = this.highestCompletedLevel[sec.id] ?? -1
                if (highest + 1 < sec.levels.length) {
                    return false
                }
            }
            return true
        },

        /** Get the next incomplete level the user should continue from. */
        continueTarget(allSections: Section[]): { sectionId: string; levelIdx: number } | null {
            const sorted = [...allSections].sort((a, b) => a.meta.order - b.meta.order)
            for (const sec of sorted) {
                if (!this.isSectionAccessible(sec.id, allSections)) continue
                const highest = this.highestCompletedLevel[sec.id] ?? -1
                if (highest + 1 < sec.levels.length) {
                    return { sectionId: sec.id, levelIdx: highest + 1 }
                }
            }
            return null
        },

        reset() {
            this.highestCompletedLevel = {}
            this.currentSection = null
            this.currentProgress = null
            save(this.$state)
        },

        /** Unlock every level in a section (debug cheat). */
        unlockAll(sectionId: string, total: number) {
            this.highestCompletedLevel[sectionId] = total - 1
            save(this.$state)
        },
    },
})
