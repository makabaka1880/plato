import 'katex/dist/katex.min.css'
import './style.scss'
import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import i18n from './i18n'
import router from './router'
import { loadSections } from '@/data'
import { useProgressStore } from '@/stores/progress'

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)
app.use(router)
app.use(i18n)
app.mount('#app')

/* ── debug cheats (browser console) ────────────────────────────────── */
;(window as any).__plato_unlockAll__ = () => {
  const store = useProgressStore(pinia)
  const sections = loadSections(i18n.global.locale.value as string)
  let total = 0
  for (const sec of sections) {
    store.unlockAll(sec.id, sec.problems.length)
    total += sec.problems.length
  }
  console.log(`[plato] all ${total} problems across ${sections.length} sections unlocked`)
}

;(window as any).__plato_goto__ = (section: string | null, problem: number) => {
  if (!section) {
    const sections = loadSections(i18n.global.locale.value as string)
    let offset = 0
    for (const sec of sections) {
      if (problem < offset + sec.problems.length) {
        router.push(`/section/${sec.id}/problem/${problem - offset}`)
        return
      }
      offset += sec.problems.length
    }
    console.log(`[plato] problem ${problem} not found`)
    return
  }
  router.push(`/section/${section}/problem/${problem}`)
}

;(window as any).__plato_help__ = () => {
  console.log('[plato] debug cheats:')
  console.log('  __plato_unlockAll()           — unlock all problems')
  console.log('  __plato_goto(section, idx)    — jump to problem (e.g. __plato_goto("propositional", 5))')
  console.log('  __plato_goto(null, globalIdx) — jump by old global index')
}
