/** localStorage keys that constitute a user's Plato progress. */
const PROGRESS_KEYS = [
    'plato-progress-v3',
    'plato-roadmap-v5',
    'plato-discoveries',
    'plato-tactics',
    'plato-locale',
] as const

export interface PlatoExport {
    version: 1
    exportedAt: string
    keys: Record<string, unknown>
}

/** Collect all progress data into an exportable object. */
export function exportProgress(): PlatoExport {
    const keys: Record<string, unknown> = {}
    for (const k of PROGRESS_KEYS) {
        const raw = localStorage.getItem(k)
        if (raw !== null) {
            try { keys[k] = JSON.parse(raw) } catch { keys[k] = raw }
        }
    }
    return {
        version: 1,
        exportedAt: new Date().toISOString(),
        keys,
    }
}

/** Write exported data back to localStorage. Returns error string on failure, null on success. */
export function importProgress(data: unknown): string | null {
    if (!data || typeof data !== 'object') return 'Invalid file: not a JSON object.'
    const obj = data as Record<string, unknown>

    if (typeof obj.version !== 'number' || obj.version < 1) return 'Unsupported export version.'
    if (!obj.keys || typeof obj.keys !== 'object') return 'Missing keys object.'

    const keys = obj.keys as Record<string, unknown>
    let count = 0
    for (const k of PROGRESS_KEYS) {
        if (k in keys) {
            localStorage.setItem(k, JSON.stringify(keys[k]))
            count++
        }
    }
    if (count === 0) return 'No recognized progress keys found in file.'
    return null
}
