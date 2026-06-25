import { defineStore } from 'pinia'

const KEY = 'plato-highest'

function load(): number {
  try {
    const raw = localStorage.getItem(KEY)
    if (raw !== null) return JSON.parse(raw) as number
  } catch { /* ignore */ }
  return -1
}

function save(n: number) {
  localStorage.setItem(KEY, JSON.stringify(n))
}

export const useProgressStore = defineStore('progress', {
  state: () => ({
    highestSolved: load(),
  }),

  actions: {
    markSolved(idx: number) {
      if (idx > this.highestSolved) {
        this.highestSolved = idx
        save(idx)
      }
    },

    reset() {
      this.highestSolved = -1
      save(-1)
    },

    /** Unlock every problem (debug cheat). */
    unlockAll(total: number) {
      this.highestSolved = total - 1
      save(this.highestSolved)
    },
  },
})
