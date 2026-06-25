import { defineStore } from 'pinia'
import { resolveGlobalIndex } from '@/data'
import type { Section } from '@/types'

const OLD_KEY = 'plato-highest'
const KEY = 'plato-progress-v2'

interface ProgressState {
    highestSolved: Record<string, number>
    currentSection: string | null
}

function migrateOld(): ProgressState | null {
    try {
        const raw = localStorage.getItem(OLD_KEY)
        if (raw === null) return null
        const oldHighest = JSON.parse(raw) as number
        if (oldHighest < 0) return null

        // Map old global index to per-section progress
        const result: Record<string, number> = {}
        // Iterate through all solved problems and mark them in their sections
        for (let globalIdx = 0; globalIdx <= oldHighest; globalIdx++) {
            const resolved = resolveGlobalIndex(globalIdx)
            if (resolved) {
                const cur = result[resolved.sectionId] ?? -1
                if (resolved.sectionIdx > cur) {
                    result[resolved.sectionId] = resolved.sectionIdx
                }
            }
        }
        // Determine current section (the one containing oldHighest)
        const last = resolveGlobalIndex(oldHighest)

        // Clean up old key after migration
        localStorage.removeItem(OLD_KEY)
        return { highestSolved: result, currentSection: last?.sectionId ?? null }
    } catch { /* ignore */ }
    return null
}

function load(): ProgressState {
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (parsed && typeof parsed.highestSolved === 'object') {
                return {
                    highestSolved: parsed.highestSolved as Record<string, number>,
                    currentSection: parsed.currentSection ?? null,
                }
            }
        }
    } catch { /* ignore */ }

    // Try migration from old format
    const migrated = migrateOld()
    if (migrated) return migrated

    return { highestSolved: {}, currentSection: null }
}

function save(state: ProgressState) {
    localStorage.setItem(KEY, JSON.stringify(state))
}

export const useProgressStore = defineStore('progress', {
    state: (): ProgressState => load(),

    actions: {
        markSolved(sectionId: string, idx: number) {
            const cur = this.highestSolved[sectionId] ?? -1
            if (idx > cur) {
                this.highestSolved[sectionId] = idx
                save(this.$state)
            }
        },

        setCurrentSection(sectionId: string) {
            this.currentSection = sectionId
            save(this.$state)
        },

        /** Check if a problem within a section is unlocked. Problem 0 is always unlocked. */
        isUnlocked(sectionId: string, sectionIdx: number): boolean {
            if (sectionIdx === 0) return true
            return sectionIdx <= (this.highestSolved[sectionId] ?? -1) + 1
        },

        /**
         * Check if a section is accessible. A section is accessible if:
         * - It's the first section (lowest order), OR
         * - All problems in all prior sections (by order) are completed
         */
        isSectionAccessible(sectionId: string, allSections: Section[]): boolean {
            const sorted = [...allSections].sort((a, b) => a.meta.order - b.meta.order)
            const targetIdx = sorted.findIndex(s => s.id === sectionId)
            if (targetIdx < 0) return false
            if (targetIdx === 0) return true

            // Check that all prior sections are fully completed
            for (let i = 0; i < targetIdx; i++) {
                const sec = sorted[i]!
                const highest = this.highestSolved[sec.id] ?? -1
                if (highest + 1 < sec.problems.length) {
                    return false // prior section not fully completed
                }
            }
            return true
        },

        /** Get the section the user should continue from (first with unsolved problems). */
        continueTarget(allSections: Section[]): { sectionId: string; sectionIdx: number } | null {
            const sorted = [...allSections].sort((a, b) => a.meta.order - b.meta.order)
            for (const sec of sorted) {
                if (!this.isSectionAccessible(sec.id, allSections)) continue
                const highest = this.highestSolved[sec.id] ?? -1
                if (highest + 1 < sec.problems.length) {
                    return { sectionId: sec.id, sectionIdx: highest + 1 }
                }
            }
            return null
        },

        reset() {
            this.highestSolved = {}
            this.currentSection = null
            save(this.$state)
        },

        /** Unlock every problem in a section (debug cheat). */
        unlockAll(sectionId: string, total: number) {
            this.highestSolved[sectionId] = total - 1
            save(this.$state)
        },
    },
})
