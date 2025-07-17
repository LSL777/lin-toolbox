<script lang="ts" setup>
import {WebviewWindow} from '@tauri-apps/api/webviewWindow';
import {nextTick, onMounted, ref} from 'vue';

const windowTitle = ref("Lin's Toolbox");
const appWindow = WebviewWindow.getCurrent()

// 窗口是否处于最大化状态 true：是 false：否
const windowMaximized = ref(false)

/**
 * 恢复窗口大小
 */
const restoreWindow = async () => {
  console.log('restoreWindow called')
  if (windowMaximized.value) {
    await appWindow.unmaximize()
  } else {
    await appWindow.maximize()
  }
}

/**
 * 处理窗口关闭
 */
const handleCloseWin = async () => {
  await nextTick(() => {
    appWindow.hide()
  })
}

onMounted(() => {
  // 监听最大化
  appWindow.onResized(async () => {
    windowMaximized.value = await appWindow.isMaximized()
  })
  // 初始化时同步一次
  appWindow.isMaximized().then(isMax => {
    windowMaximized.value = isMax
  })
})
</script>

<template>
  <div class="action-bar">
    <div class="titlebar-title">
      {{ windowTitle }}
    </div>
    <div data-tauri-drag-region class="titlebar">
      <div class="titlebar-button" id="titlebar-minimize" @click.stop="appWindow.minimize()">
        <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize"/>
      </div>
      <div class="titlebar-button" id="titlebar-maximize" @click.stop="restoreWindow">
        <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize"/>
      </div>
      <div class="titlebar-button" id="titlebar-close" @click.stop="handleCloseWin">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close"/>
      </div>
    </div>
  </div>

</template>

<style scoped>
.action-bar {
  display: flex;
  width: 100vw;
}

.titlebar-title {
  display: flex;
  align-items: center;
  width: 120px;
  height: 30px;
  color: #000000;
  font-size: 14px;
  margin-left: 10px;
  user-select: none;
  -webkit-user-select: none;
  -webkit-app-region: no-drag;
  z-index: 999;
}

.titlebar {
  flex: 1;
  height: 30px;
  background: linear-gradient(90deg, #f5e6ff 0%, #e8daff 100%);
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 998;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}

.titlebar-button:hover {
  background: #d5b0fd;
}
</style>