import { defineStore } from 'pinia'
import { resolveGlobalIndex } from '@/data'
import type { StepMeta } from '@/composables/useNlg'

export interface RoadmapEntry {
    sectionId: string
    sectionIdx: number
    /** Raw proof steps — NLG lines are generated dynamically from these at display time. */
    steps: StepMeta[]
    /** The goal formula (s-expression). */
    goal: string
}

const KEY = 'plato-roadmap-v3'

/** Old v2 entries had `proofLines: string[]` and `description: string`. */
interface V2Entry {
    sectionId?: string
    sectionIdx?: number
    idx?: number
    description: string
    goal: string
    proofLines: string[]
}

function load(): RoadmapEntry[] {
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (Array.isArray(parsed)) return parsed as RoadmapEntry[]
        }
    } catch { /* ignore */ }

    // Migrate from v2 / legacy
    return migrateOld()
}

/** Migrate from v2 (`plato-roadmap-v2`) and legacy (`plato-roadmap`). */
function migrateOld(): RoadmapEntry[] {
    const entries: RoadmapEntry[] = []

    // Try v2 first
    try {
        const v2Raw = localStorage.getItem('plato-roadmap-v2')
        if (v2Raw) {
            const v2Entries: V2Entry[] = JSON.parse(v2Raw)
            if (Array.isArray(v2Entries)) {
                for (const e of v2Entries) {
                    if (e.sectionId !== undefined && e.sectionIdx !== undefined) {
                        entries.push({
                            sectionId: e.sectionId,
                            sectionIdx: e.sectionIdx,
                            steps: [],
                            goal: e.goal ?? '',
                        })
                    }
                }
            }
            localStorage.removeItem('plato-roadmap-v2')
        }
    } catch { /* ignore */ }

    // Try legacy (global idx)
    if (entries.length === 0) {
        try {
            const oldRaw = localStorage.getItem('plato-roadmap')
            if (oldRaw) {
                const oldEntries = JSON.parse(oldRaw)
                if (Array.isArray(oldEntries)) {
                    for (const e of oldEntries) {
                        if (typeof e.idx !== 'number') continue
                        const resolved = resolveGlobalIndex(e.idx)
                        if (resolved) {
                            entries.push({
                                sectionId: resolved.sectionId,
                                sectionIdx: resolved.sectionIdx,
                                steps: [],
                                goal: e.goal ?? '',
                            })
                        }
                    }
                }
                localStorage.removeItem('plato-roadmap')
            }
        } catch { /* ignore */ }
    }

    return entries
}

function save(entries: RoadmapEntry[]) {
    localStorage.setItem(KEY, JSON.stringify(entries))
}

export const useRoadmapStore = defineStore('roadmap', {
    state: () => ({
        entries: load() as RoadmapEntry[],
    }),

    getters: {
        /** Group entries by section ID. */
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
