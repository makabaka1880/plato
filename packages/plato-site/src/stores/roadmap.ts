import { defineStore } from 'pinia'
import { resolveGlobalIndex } from '@/data'
import type { StepMeta } from '@/composables/useNlg'

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

const KEY = 'plato-roadmap-v5'
const V4_KEY = 'plato-roadmap-v4'

/** Old v4 entries — untagged, always problem-type. */
interface V4Entry {
    sectionId: string
    sectionIdx: number
    steps: StepMeta[]
    goal: string
}

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
    // Try v5 first
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (Array.isArray(parsed)) return parsed as RoadmapEntry[]
        }
    } catch { /* ignore */ }

    // Migrate from v4 (untagged problem entries)
    const v4 = migrateV4()
    if (v4.length > 0) return v4

    // Migrate from older formats
    return migrateOld()
}

/** Migrate from v4: wrap problem entries with `tag: 'problem'` and rename `steps` → `proof`. */
function migrateV4(): RoadmapEntry[] {
    try {
        const raw = localStorage.getItem(V4_KEY)
        if (raw) {
            const entries: any[] = JSON.parse(raw)
            if (Array.isArray(entries)) {
                const result: RoadmapEntry[] = entries.map(e => ({
                    sectionId: e.sectionId as string,
                    sectionIdx: e.sectionIdx as number,
                    tag: 'problem' as const,
                    goal: e.goal as string ?? '',
                    proof: (e.steps ?? []) as StepMeta[],
                }))
                localStorage.removeItem(V4_KEY)
                return result
            }
        }
    } catch { /* ignore */ }
    return []
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
                            sectionIdx: e.sectionIdx + 1, // problem index → level index
                            tag: 'problem',
                            goal: e.goal ?? '',
                            proof: [],
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
                                sectionIdx: resolved.sectionIdx + 1, // problem index → level index
                                tag: 'problem',
                                goal: e.goal ?? '',
                                proof: [],
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
