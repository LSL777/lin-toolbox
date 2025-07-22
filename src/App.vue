<script setup lang="ts">
import {onMounted, onUnmounted, ref} from 'vue';
import {getCurrentWebviewWindow} from '@tauri-apps/api/webviewWindow';
import {UnlistenFn} from "@tauri-apps/api/event";
import Database from "@tauri-apps/plugin-sql";

interface TodoItem {
  todo_id: string;
  content: string;
  trigger_time: string;
  task_type: number;
}

let unlisten: UnlistenFn | null = null;

const db = ref<Database | null>(null);

const updateStatus = async (id: any, type: number) => {
  if (type === 0) {
    console.log("更新非周期性任务状态")
    await db.value?.execute("update todo_list set status = $1 where id = $2", [2, id])
  }
}

onMounted(async () => {
  const appWebview = getCurrentWebviewWindow();
  unlisten = await appWebview.listen('reminder_triggered', (event) => {
    console.log('全局监听到提醒事件:', event);
    const payload: TodoItem = event.payload as TodoItem;
    updateStatus(payload.todo_id, payload.task_type)
  });
  try {
    db.value = await Database.load('sqlite:test.db');
  } catch (error) {
    console.error('数据库加载失败:', error);
  }
});

// 组件卸载时移除监听器
onUnmounted(() => {
  if (unlisten) {
    unlisten();
    console.log('事件监听器已移除');
  }
});
</script>

<template>
  <div>
    <router-view/>
  </div>
</template>

<style scoped>

</style>