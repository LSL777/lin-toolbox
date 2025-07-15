import {createRouter, createWebHistory} from "vue-router";
import Layout from "@/layout/Layout.vue";
import {Router} from "vue-router";

const router: Router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'Layout',
            meta: {
                name: 'Layout'
            },
            component: Layout,
            redirect: '/build_info',
            children: [
                {
                    path: '/build_info',
                    name: 'buildInfo',
                    meta: {
                        name: '生成信息'
                    },
                    component: () => import('@/views/info/BuildInfo.vue')
                },
                {
                    path: '/json',
                    name: 'json',
                    meta: {
                        name: 'json工具'
                    },
                    component: () => import('@/views/json/JsonTool.vue')
                }
            ]
        }
    ]
})

export default router