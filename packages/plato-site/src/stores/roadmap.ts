import { defineStore } from 'pinia'
import type { StepMeta } from '@/composables/useNlg'

export const ROADMAP_KEY = 'plato-roadmap-v5'

export type RoadmapEntry =
    | {
        sectionId: string
        sectionIdx: number
        tag: 'problem'
        goal: string
        proof: StepMeta[]
    }
    | {
        sectionId: string
        sectionIdx: number
        tag: 'discovery'
        /** Last dialogue line index the user reached (0-based). */
        idx: number
    }


function load(): RoadmapEntry[] {
    try {
        const raw = localStorage.getItem(ROADMAP_KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (Array.isArray(parsed)) return parsed as RoadmapEntry[]
        }
    } catch { /* ignore */ }
    return []
}

function save(entries: RoadmapEntry[]) {
    localStorage.setItem(ROADMAP_KEY, JSON.stringify(entries))
}

export const useRoadmapStore = defineStore('roadmap', {
    state: () => ({
        entries: load() as RoadmapEntry[],
    }),

    getters: {
        bySection: (state): Record<string, RoadmapEntry[]> => {
            const map: Record<string, RoadmapEntry[]> = {}
            for (const e of state.entries) {
                if (!map[e.sectionId]) map[e.sectionId] = []
                map[e.sectionId]!.push(e)
            }
            return map
        },
    },

    actions: {
        add(entry: RoadmapEntry) {
            const existing = this.entries.findIndex(
                e => e.sectionId === entry.sectionId && e.sectionIdx === entry.sectionIdx,
            )
            if (existing >= 0) {
                this.entries[existing] = entry
            } else {
                this.entries.push(entry)
            }
            save(this.entries)
        },

        reset() {
            this.entries = []
            save(this.entries)
        },
    },
})
