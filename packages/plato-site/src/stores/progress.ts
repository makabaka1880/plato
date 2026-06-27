import { defineStore } from 'pinia'
import type { Section } from '@/types'
import type { StepMeta } from '@/composables/useNlg'

export const PROGRESS_KEY = 'plato-progress-v3'

export type CurrentProgress =
    | { sectionId: string; levelIdx: number; tag: 'problem'; proof: StepMeta[] }
    | { sectionId: string; levelIdx: number; tag: 'discovery'; idx: number }

interface ProgressState {
    highestCompletedLevel: Record<string, number>
    currentSection: string | null
    currentProgress: CurrentProgress | null
}

function load(): ProgressState {
    try {
        const raw = localStorage.getItem(PROGRESS_KEY)
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

    return { highestCompletedLevel: {}, currentSection: null, currentProgress: null }
}

function save(state: ProgressState) {
    localStorage.setItem(PROGRESS_KEY, JSON.stringify(state))
}

export const useProgressStore = defineStore('progress', {
    state: (): ProgressState => load(),

    actions: {
        markCompleted(sectionId: string, levelIdx: number) {
            const cur = this.highestCompletedLevel[sectionId] ?? -1
            if (levelIdx > cur) {
                this.highestCompletedLevel[sectionId] = levelIdx
            }
            if (this.currentProgress && this.currentProgress.sectionId === sectionId && this.currentProgress.levelIdx === levelIdx) {
                this.currentProgress = null
            }
            save(this.$state)
        },

        setCurrentSection(sectionId: string) {
            this.currentSection = sectionId
            save(this.$state)
        },

        saveCurrentProgress(cp: CurrentProgress) {
            this.currentProgress = cp
            save(this.$state)
        },

        clearCurrentProgress() {
            this.currentProgress = null
            save(this.$state)
        },

        isUnlocked(sectionId: string, levelIdx: number): boolean {
            if (levelIdx === 0) return true
            return levelIdx <= (this.highestCompletedLevel[sectionId] ?? -1) + 1
        },

        isSectionAccessible(sectionId: string, allSections: Section[]): boolean {
            const sorted = [...allSections].sort((a, b) => a.meta.order - b.meta.order)
            const targetIdx = sorted.findIndex(s => s.id === sectionId)
            if (targetIdx < 0) return false
            if (targetIdx === 0) return true

            for (let i = 0; i < targetIdx; i++) {
                const sec = sorted[i]!
                const highest = this.highestCompletedLevel[sec.id] ?? -1
                if (highest + 1 < sec.levels.length) {
                    return false
                }
            }
            return true
        },

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

        unlockAll(sectionId: string, total: number) {
            this.highestCompletedLevel[sectionId] = total - 1
            save(this.$state)
        },
    },
})
