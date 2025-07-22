import '@/assets/iconfont/iconfont.css'
import 'element-plus/dist/index.css'
import '@/style/main.css'
import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import router from "@/router";
import copy from "@/directives/copy.ts";
import zhCn from 'element-plus/es/locale/lang/zh-cn'

const app = createApp(App)
app.use(ElementPlus, { size: 'default', zIndex: 3000, locale: zhCn, })
    .use(router)
    .directive('copy', copy)
    .mount("#app")
