export interface Segment {
  type: 'text' | 'latex' | 'bold' | 'code'
  value: string
  key: number
}

/**
 * Parse a string with **bold**, `code`, and $latex$ into segments.
 */
export function parseInline(text: string): Segment[] {
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
    // **bold**
    if (text[i] === '*' && text[i + 1] === '*') {
      const end = text.indexOf('**', i + 2)
      if (end !== -1) {
        segs.push({ type: 'bold', value: text.slice(i + 2, end), key: key++ })
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
    // plain text — accumulate until next special char
    let j = i
    while (j < text.length && text[j] !== '$' && text[j] !== '*' && text[j] !== '`') {
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
