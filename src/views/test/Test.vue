<script setup lang="ts">
import {sendNotification} from "@tauri-apps/plugin-notification";
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {dayjs} from "element-plus";

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
  sendNotification({title: 'Tauri', body: 'Tauri is awesome!'});
}
const snowId = ref()

const getSnowId = async () => {
  snowId.value = await invoke("generate_snowflake_id")
}

interface NetResult {
  result: Boolean;
  reason: string;
}

const testOpenPort = async () => {
  const res: NetResult = await invoke('is_port_open', {host: "8.138.224.34", port: 3306})
  console.log(res.result)
  console.log(res.reason)

  try {
    await invoke('is_port_open', {host: "8.138.224.34", port: 3305})
  } catch (error) {
    console.log(error)
  }
}

const createRemindTask = async () => {
  const todo = {
    id: 123,
    content: "测试任务",
    remind_time: dayjs("2025-07-22 10:42:00").format("YYYY-MM-DD HH:mm:ss"),
  };
  await invoke('schedule_reminder', {todo: todo});
}

const sendNotificationOnRust = async () => {
  await invoke("send_notification")
}

const operateDatabaseWithRust = async () => {
  await invoke("select_test")
}

const createCronTask = async () => {
  const task = {
    id: "555",
    content: "String",
    cron_expr: "0/30 * * * * ?",
  };
  await invoke("schedule_cron_task", {task: task})
}
</script>

<template>
  <el-button @click.stop="testNotice">发送通知</el-button>
  <el-button @click.stop="startFlashingTrayIcon">闪烁</el-button>
  <el-button @click.stop="getSnowId">生成ID</el-button>
  <el-button @click.stop="testOpenPort">测试端口</el-button>
  <el-button @click.stop="createRemindTask">创建提醒任务</el-button>
  <el-button @click.stop="sendNotificationOnRust">测试rust调用系统通知</el-button>
  <el-button @click.stop="operateDatabaseWithRust">rust操作数据库</el-button>
  <el-button @click.stop="createCronTask">创建cron任务</el-button>
  <el-input v-model="snowId"></el-input>
</template>

<style scoped>

</style>