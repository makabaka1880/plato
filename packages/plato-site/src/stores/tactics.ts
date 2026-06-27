import { defineStore } from 'pinia'

export const TACTICS_KEY = 'plato-tactics'

function load(): string[] {
    try {
        const raw = localStorage.getItem(TACTICS_KEY)
        if (raw !== null) return JSON.parse(raw)
    } catch { /* ignore */ }
    return []
}

function save(names: string[]) {
    localStorage.setItem(TACTICS_KEY, JSON.stringify(names))
}

export const useTacticsStore = defineStore('tactics', {
    state: () => ({
        collected: load() as string[],
    }),
    actions: {
        add(name: string) {
            if (!this.collected.includes(name)) {
                this.collected.push(name)
                save(this.collected)
            }
        },
        reset() {
            this.collected = []
            save(this.collected)
        },
    },
})
