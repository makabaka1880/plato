import { defineStore } from 'pinia'

export const DISCOVERY_KEY = 'plato-discoveries'

interface DiscoveryState {
    viewed: Record<string, boolean>
    position: Record<string, number>
}

function load(): DiscoveryState {
    try {
        const raw = localStorage.getItem(DISCOVERY_KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (parsed && typeof parsed.viewed === 'object') {
                return {
                    viewed: parsed.viewed as Record<string, boolean>,
                    position: (parsed.position as Record<string, number>) ?? {},
                }
            }
        }
    } catch { /* ignore */ }
    return { viewed: {}, position: {} }
}

function save(state: DiscoveryState) {
    localStorage.setItem(DISCOVERY_KEY, JSON.stringify(state))
}

export const useDiscoveryStore = defineStore('discovery', {
    state: (): DiscoveryState => load(),

    actions: {
        markViewed(sectionId: string, levelIdx: number) {
            this.viewed[this.posKey(sectionId, levelIdx)] = true
            save(this.$state)
        },

        isViewed(sectionId: string, levelIdx: number): boolean {
            return this.viewed[this.posKey(sectionId, levelIdx)] ?? false
        },

        posKey(sectionId: string, levelIdx: number): string {
            return `${sectionId}/${levelIdx}`
        },

        getPosition(sectionId: string, levelIdx: number): number {
            return this.position[this.posKey(sectionId, levelIdx)] ?? 0
        },

        setPosition(sectionId: string, levelIdx: number, idx: number) {
            this.position[this.posKey(sectionId, levelIdx)] = idx
            save(this.$state)
        },

        reset() {
            this.viewed = {}
            this.position = {}
            save(this.$state)
        },
    },
})
