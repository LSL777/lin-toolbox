<script setup lang="ts">
import {sendNotification} from "@tauri-apps/plugin-notification";
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

onMounted(() => {
  const str = "620922200110222167";
  const regex = /(^[1-9]\d{5}(18|19|([23]\d))\d{2}((0[1-9])|(10|11|12))(([0-2][1-9])|10|20|30|31)\d{3}[0-9Xx]$)|(^[1-9]\d{5}\d{2}((0[1-9])|(10|11|12))(([0-2][1-9])|10|20|30|31)\d{2}$)/g; // 匹配连续数字
  console.log(str);
  console.log(regex);
  const matches = str.match(regex);
  console.log(matches); // 输出: ['123', '456']
})

const startFlashingTrayIcon = () => {
  invoke('toggle_tray_icon')
      .then(() => {
        console.log('Tray icon flashing started');
      })
      .catch((error) => {
        console.error('Error starting tray icon flashing:', error);
      });
}


const testNotice = () => {
  sendNotification({ title: 'Tauri', body: 'Tauri is awesome!' });
}
const snowId = ref()

const getSnowId = async () => {
  snowId.value = await invoke("generate_snowflake_id")
}
</script>

<template>
  <el-button @click.stop="testNotice">发送通知</el-button>
  <el-button @click.stop="startFlashingTrayIcon">闪烁</el-button>
  <el-button @click.stop="getSnowId">生成ID</el-button>
  <el-input v-model="snowId"></el-input>
</template>

<style scoped>

</style>