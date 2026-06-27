import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import LevelView from '@/views/LevelView.vue'
import CustomProblemView from '@/views/CustomProblemView.vue'
import AboutView from '@/views/AboutView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import LockedView from '@/views/LockedView.vue'
import { resolveGlobalIndex } from '@/data'

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: '/', name: 'home', component: HomeView },
        {
            path: '/section/:sectionId/level/:idx',
            name: 'level',
            component: LevelView,
            props: r => ({
                sectionId: r.params.sectionId,
                levelIdx: Number(r.params.idx),
            }),
        },
        // Legacy redirect: /section/:id/problem/:idx → /section/:id/level/:idx+1
        {
            path: '/section/:sectionId/problem/:idx',
            redirect: (to) => {
                const idx = Number(to.params.idx)
                if (!Number.isFinite(idx) || idx < 0) return { name: 'home' }
                // Old problem 0 = new level 1 (level 0 is the discovery)
                return `/section/${to.params.sectionId}/level/${idx + 1}`
            },
        },
        // Legacy redirect: /section/:id/discovery → /section/:id/level/0
        {
            path: '/section/:sectionId/discovery',
            redirect: (to) => `/section/${to.params.sectionId}/level/0`,
        },
        { path: '/custom', name: 'custom', component: CustomProblemView },
        { path: '/about', name: 'about', component: AboutView },
        { path: '/locked', name: 'locked', component: LockedView },
        // Legacy redirect: /problem/:idx → resolved to level route
        {
            path: '/problem/:idx',
            redirect: (to) => {
                const idx = Number(to.params.idx)
                if (!Number.isFinite(idx) || idx < 0) return { name: 'home' }
                const resolved = resolveGlobalIndex(idx)
                if (resolved) {
                    // resolveGlobalIndex returns 0-based problem index; add +1 for level index
                    return `/section/${resolved.sectionId}/level/${resolved.sectionIdx + 1}`
                }
                return { name: 'notFound' }
            },
        },
        { path: '/:pathMatch(.*)*', name: 'notFound', component: NotFoundView },
    ],
})

export default router
