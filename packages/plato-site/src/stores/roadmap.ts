import { defineStore } from 'pinia'
import { resolveGlobalIndex } from '@/data'

export interface RoadmapEntry {
    sectionId: string
    sectionIdx: number
    description: string
    goal: string
    proofLines: string[]
}

const OLD_KEY = 'plato-roadmap'
const KEY = 'plato-roadmap-v2'

/** Migrate from old format (global `idx`) to new format (`sectionId` + `sectionIdx`). */
function migrateOld(): RoadmapEntry[] | null {
    try {
        const raw = localStorage.getItem(OLD_KEY)
        if (raw === null) return null
        const oldEntries = JSON.parse(raw)
        if (!Array.isArray(oldEntries) || oldEntries.length === 0) return null

        const newEntries: RoadmapEntry[] = []
        for (const old of oldEntries) {
            if (typeof old.idx !== 'number') continue
            const resolved = resolveGlobalIndex(old.idx)
            if (resolved) {
                newEntries.push({
                    sectionId: resolved.sectionId,
                    sectionIdx: resolved.sectionIdx,
                    description: old.description ?? '',
                    goal: old.goal ?? '',
                    proofLines: old.proofLines ?? [],
                })
            }
        }
        // Clean up old key
        localStorage.removeItem(OLD_KEY)
        return newEntries.length > 0 ? newEntries : null
    } catch { /* ignore */ }
    return null
}

function load(): RoadmapEntry[] {
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (Array.isArray(parsed)) return parsed as RoadmapEntry[]
        }
    } catch { /* ignore */ }

    // Try migration
    const migrated = migrateOld()
    if (migrated) return migrated

    return []
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
