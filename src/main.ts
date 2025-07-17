import '@/assets/iconfont/iconfont.css'
import 'element-plus/dist/index.css'
import '@/style/main.css'
import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import router from "@/router";
import copy from "@/directives/copy.ts";

const app = createApp(App)
app.use(ElementPlus, { size: 'default', zIndex: 3000 })
    .use(router)
    .directive('copy', copy)
    .mount("#app")
