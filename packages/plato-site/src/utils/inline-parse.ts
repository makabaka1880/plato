export interface Segment {
    type: 'text' | 'latex' | 'bold' | 'code' | 'glossary'
    value: string
    key: number
    /** Display text for glossary links (when different from the id), e.g. "implies" → id="implication" */
    display?: string
    /** Nested children — only used when type='bold' and its content contains other segments */
    children?: Segment[]
}

/**
 * Parse a string with **bold**, `code`, $latex$, and [glossary] into flat segments.
 * Bold regions that contain other markup are recursively parsed.
 */
export function parseInline(text: string): Segment[] {
    return parseFlat(text)
}

function parseFlat(text: string): Segment[] {
    const segs: Segment[] = []
    let key = 0
    let i = 0
    while (i < text.length) {
        // $latex$
        if (text[i] === '$') {
            const end = text.indexOf('$', i + 1)
            if (end !== -1) {
                segs.push({ type: 'latex', value: text.slice(i + 1, end), key: key++ })
                i = end + 1
                continue
            }
        }
        // **bold** — recursively parse inner content so [links] inside bold work
        if (text[i] === '*' && text[i + 1] === '*') {
            const end = text.indexOf('**', i + 2)
            if (end !== -1) {
                const inner = text.slice(i + 2, end)
                const children = parseFlat(inner)
                const hasMarkup = children.some(c => c.type !== 'text')
                if (hasMarkup) {
                    // Rich bold: use children so the template can recurse
                    segs.push({ type: 'bold', value: '', key: key++, children })
                } else {
                    // Plain bold: just the text
                    segs.push({ type: 'bold', value: inner, key: key++ })
                }
                i = end + 2
                continue
            }
        }
        // `code`
        if (text[i] === '`') {
            const end = text.indexOf('`', i + 1)
            if (end !== -1) {
                segs.push({ type: 'code', value: text.slice(i + 1, end), key: key++ })
                i = end + 1
                continue
            }
        }
        // [glossary term] or [id|display text]
        if (text[i] === '[') {
            const end = text.indexOf(']', i + 1)
            if (end !== -1 && end - i <= 80) {
                const raw = text.slice(i + 1, end)
                const pipe = raw.indexOf('|')
                if (pipe !== -1) {
                    segs.push({ type: 'glossary', value: raw.slice(0, pipe), display: raw.slice(pipe + 1), key: key++ })
                } else {
                    segs.push({ type: 'glossary', value: raw, key: key++ })
                }
                i = end + 1
                continue
            }
        }
        // plain text — accumulate until next special char
        let j = i
        while (j < text.length && text[j] !== '$' && text[j] !== '*' && text[j] !== '`' && text[j] !== '[') {
            j++
        }
        // also stop at `**`
        if (j < text.length && text[j] === '*' && text[j + 1] !== '*') {
            j++
        }
        segs.push({ type: 'text', value: text.slice(i, j), key: key++ })
        i = j
    }
    return segs
}
