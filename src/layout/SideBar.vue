<template>
  <div class="sidebar">
    <div class="nav-logo">
      <img src="../assets/lyy.png" alt="">
      <h3 style="text-align: center;color: #555555">无聊工具箱</h3>
    </div>
    <el-menu :default-active="'/build_info'" router :collapse="isCollapse" background-color="#F3E9FF"
             text-color="#fff"
             active-text-color="#8A77C1">
      <el-menu-item v-for="item in menuList" :key="item.id" :index="item.url">
        <el-icon>
          <IconFont :name="item.icon" />
        </el-icon>
        <span>{{item.title}}</span>
      </el-menu-item>
<!--      <el-menu-item index="/build_info">-->
<!--        <el-icon>-->
<!--          <IconFont name="xinxi"/>-->
<!--        </el-icon>-->
<!--        <span>生成信息</span>-->
<!--      </el-menu-item>-->
<!--      <el-menu-item index="/json">-->
<!--        <el-icon>-->
<!--          <IconFont name="json"/>-->
<!--        </el-icon>-->
<!--        <span>JSON工具</span>-->
<!--      </el-menu-item>-->
<!--      <el-menu-item index="/regex">-->
<!--        <el-icon>-->
<!--          <IconFont name="zhengzebiaodashi_1"/>-->
<!--        </el-icon>-->
<!--        <span>正则表达式</span>-->
<!--      </el-menu-item>-->
<!--      <el-menu-item index="/network">-->
<!--        <el-icon>-->
<!--          <IconFont name="jieruwangluodaojisuanji"/>-->
<!--        </el-icon>-->
<!--        <span>网络工具</span>-->
<!--      </el-menu-item>-->
<!--      <el-menu-item index="/test">-->
<!--        <el-icon>-->
<!--          <IconFont name="ceshi"/>-->
<!--        </el-icon>-->
<!--        <span>测试页面</span>-->
<!--      </el-menu-item>-->
    </el-menu>
  </div>
</template>

<script setup>
import  { ref } from 'vue'
import IconFont from "@/components/IconFont.vue";
const isCollapse = ref(false)

const menuList = [
  {
    id: 1,
    title: '生成信息',
    url: '/build_info',
    icon: 'xinxi'
  },
  {
    id: 2,
    title: 'JSON工具',
    url: '/json',
    icon: 'json'
  },
  {
    id: 3,
    title: '正则表达式',
    url: '/regex',
    icon: 'zhengzebiaodashi_1'
  },
  {
    id: 4,
    title: '网络工具',
    url: '/network',
    icon: 'jieruwangluodaojisuanji'
  },
  {
    id: 5,
    title: '待办事项',
    url: '/todo_list',
    icon: 'daibanshixiang'
  },
  {
    id: 6,
    title: '测试页面',
    url: '/test',
    icon: 'ceshi'
  }
]
</script>

<style scoped>
.nav-logo {
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-logo > img {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  margin-right: 4px;
}

.sidebar {
  position: fixed;
  left: 0;
  top: 30px;
  width: 160px;
  height: calc(100vh - 30px);
  overflow-y: auto;
  background-color: #F3E9FF;
}

/* 隐藏滚动条 - Chrome、Safari 和新版 Edge */
.sidebar::-webkit-scrollbar {
  display: none;
}

:deep(.el-menu) {
  border-right: none;
}

:deep(.el-menu-item) {
  color: #555555;
}

:deep(.el-menu-item.is-active) {
  color: #8A77C1 !important;
  background-color: #e0d0f7 !important;
  font-weight: bold;
}
</style>