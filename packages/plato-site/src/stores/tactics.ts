import { defineStore } from 'pinia'
import type { Tactic } from '@/types'

const KEY = 'plato-tactics'

function load(): Map<string, Tactic> {
  try {
    const raw = localStorage.getItem(KEY)
    if (raw !== null) return new Map(JSON.parse(raw))
  } catch { /* ignore */ }
  return new Map()
}

function save(collected: Map<string, Tactic>) {
  localStorage.setItem(KEY, JSON.stringify(Array.from(collected.entries())))
}

export const useTacticsStore = defineStore('tactics', {
  state: () => ({
    collected: load(),
  }),
  getters: {
    all(state): Tactic[] {
      return Array.from(state.collected.values())
    },
  },
  actions: {
    add(tactic: Tactic) {
      if (!this.collected.has(tactic.name)) {
        this.collected.set(tactic.name, tactic)
        save(this.collected)
      }
    },
    reset() {
      this.collected.clear()
      save(this.collected)
    },
  },
})
