import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import ProblemView from '@/views/ProblemView.vue'
import CustomProblemView from '@/views/CustomProblemView.vue'
import AboutView from '@/views/AboutView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import LockedView from '@/views/LockedView.vue'
import DiscoveryView from '@/views/DiscoveryView.vue'
import { resolveGlobalIndex } from '@/data'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'home', component: HomeView },
    {
      path: '/section/:sectionId/problem/:idx',
      name: 'problem',
      component: ProblemView,
      props: r => ({
        sectionId: r.params.sectionId,
        problemIdx: Number(r.params.idx),
      }),
    },
    {
      path: '/section/:sectionId/discovery',
      name: 'discovery',
      component: DiscoveryView,
      props: r => ({ sectionId: r.params.sectionId }),
    },
    { path: '/custom', name: 'custom', component: CustomProblemView },
    { path: '/about', name: 'about', component: AboutView },
    { path: '/locked', name: 'locked', component: LockedView },
    // Legacy redirect: /problem/:idx → resolved directly to section route
    {
      path: '/problem/:idx',
      redirect: (to) => {
        const idx = Number(to.params.idx)
        if (!Number.isFinite(idx) || idx < 0) return { name: 'home' }
        const resolved = resolveGlobalIndex(idx)
        if (resolved) {
          return `/section/${resolved.sectionId}/problem/${resolved.sectionIdx}`
        }
        return { name: 'notFound' }
      },
    },
    { path: '/:pathMatch(.*)*', name: 'notFound', component: NotFoundView },
  ],
})

export default router
