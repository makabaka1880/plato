import { defineStore } from 'pinia'
import type { Tactic } from '@/types'

export const useTacticsStore = defineStore('tactics', {
  state: () => ({
    collected: new Map<string, Tactic>(),
  }),
  getters: {
    all(state): Tactic[] {
      return Array.from(state.collected.values())
    },
  },
  actions: {
    add(tactic: Tactic) {
      // Dedupe by name
      if (!this.collected.has(tactic.name)) {
        this.collected.set(tactic.name, tactic)
      }
    },
    reset() {
      this.collected.clear()
    },
  },
})
