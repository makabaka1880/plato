import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import LevelView from '@/views/LevelView.vue'
import CustomProblemView from '@/views/CustomProblemView.vue'
import AboutView from '@/views/AboutView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import LockedView from '@/views/LockedView.vue'

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

        { path: '/custom', name: 'custom', component: CustomProblemView },
        { path: '/about', name: 'about', component: AboutView },
        { path: '/locked', name: 'locked', component: LockedView },
        { path: '/:pathMatch(.*)*', name: 'notFound', component: NotFoundView },
    ],
})

export default router
