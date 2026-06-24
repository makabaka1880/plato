import type { Problem } from '@/types'

/** Omit `id` from the JSON schema — it's assigned here. */
type ProblemData = Omit<Problem, 'id'>

const modules = import.meta.glob<{ default: ProblemData }>(
  './*.json',
  { eager: true },
)

/** Sorted by filename so `01-identity.json` becomes id=1, etc. */
const paths = Object.keys(modules).sort()

export const problems: Problem[] = paths.map((path, index) => ({
  id: index + 1,
  ...modules[path].default,
}))
