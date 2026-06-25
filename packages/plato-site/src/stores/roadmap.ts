import { defineStore } from 'pinia'

export interface RoadmapEntry {
  idx: number
  description: string
  goal: string
  proofLines: string[]
}

const KEY = 'plato-roadmap'

function load(): RoadmapEntry[] {
  try {
    const raw = localStorage.getItem(KEY)
    if (raw !== null) return JSON.parse(raw)
  } catch { /* ignore */ }
  return []
}

function save(entries: RoadmapEntry[]) {
  localStorage.setItem(KEY, JSON.stringify(entries))
}

export const useRoadmapStore = defineStore('roadmap', {
  state: () => ({
    entries: load() as RoadmapEntry[],
  }),
  actions: {
    add(entry: RoadmapEntry) {
      // Dedupe by idx
      const existing = this.entries.findIndex(e => e.idx === entry.idx)
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
