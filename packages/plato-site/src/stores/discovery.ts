import { defineStore } from 'pinia'

const KEY = 'plato-discoveries'

interface DiscoveryState {
    viewed: Record<string, boolean>
    position: Record<string, number>
}

function load(): DiscoveryState {
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (parsed && typeof parsed.viewed === 'object') {
                const viewed: Record<string, boolean> = {}
                // Migrate old keys (sectionId → sectionId/0) and keep new keys as-is
                for (const [k, v] of Object.entries(parsed.viewed as Record<string, boolean>)) {
                    if (k.includes('/')) {
                        viewed[k] = v
                    } else {
                        // Old format: plain sectionId → migrate to sectionId/0
                        viewed[`${k}/0`] = v
                    }
                }
                return {
                    viewed,
                    position: (parsed.position as Record<string, number>) ?? {},
                }
            }
        }
    } catch { /* ignore */ }
    return { viewed: {}, position: {} }
}

function save(state: DiscoveryState) {
    localStorage.setItem(KEY, JSON.stringify(state))
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
