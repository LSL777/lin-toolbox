<script setup lang="ts">
import {onMounted, onUnmounted, ref} from 'vue';
import {getCurrentWebviewWindow} from '@tauri-apps/api/webviewWindow';
import {UnlistenFn} from "@tauri-apps/api/event";
import Database from "@tauri-apps/plugin-sql";
import {dayjs, ElMessage} from "element-plus";
import {invoke} from "@tauri-apps/api/core";

interface TodoItem {
  todo_id: string;
  content: string;
  trigger_time: string;
  task_type: number;
}

/**
 * 定时任务对象
 */
interface TodoTask {
  id: string;
  content: string;
  remind_time: string;
  cron_expr: String;
}

let unlisten: UnlistenFn | null = null;

const db = ref<Database | null>(null);

const cronTaskList = ref<TodoTask[]>([])
const aperiodicityTask = ref<TodoTask[]>([])

const initCronTask = async () => {
  try {
    cronTaskList.value = (await db.value?.select("select id, content, cron_expression as cron_expr from todo_list where is_recurring = 1 and status = 1 and effective = 1")) as TodoTask[] || []
    if (cronTaskList.value.length > 0) {
      for (let taskItem of cronTaskList.value) {
        const task: TodoTask = {
          id: (BigInt(taskItem.id)).toString(),
          content: taskItem.content,
          cron_expr: taskItem.cron_expr,
          remind_time: ''
        };
        await invoke('schedule_cron_task', {task: task});
      }
    }
  } catch (e) {
    console.log(e)
    ElMessage({type: 'error', message: `初始化周期性任务失败, ${e}`})
  }
}

const initAperiodicityTask = async () => {
  const currentTime = dayjs().format('YYYY-MM-DD HH:mm:ss')
  try {
    aperiodicityTask.value = (await db.value?.select(`select id, content, remind_time
                                                      from todo_list
                                                      where is_recurring = 0
                                                        and status = 0
                                                        and effective = 1
                                                        and remind_time >= ${'\'' + currentTime + '\''}`)) as TodoTask[] || []
    console.log(aperiodicityTask.value.length)
    if (aperiodicityTask.value.length > 0) {
      for (let taskItem of aperiodicityTask.value) {
        const task: TodoTask = {
          id: (BigInt(taskItem.id)).toString(),
          content: taskItem.content,
          cron_expr: taskItem.cron_expr,
          remind_time: taskItem.remind_time
        };
        await invoke('schedule_reminder', {todo: task});
      }
    }
  } catch (e) {
    console.log(e)
    ElMessage({type: 'error', message: `初始化非周期性任务失败, ${e}`})
  }
}

const updateStatus = async (id: any, type: number) => {
  if (type === 0) {
    console.log("更新非周期性任务状态")
    await db.value?.execute("update todo_list set status = $1, effective = $2, update_time = $3 where id = $4",
        [2, 0, dayjs().format('YYYY-MM-DD HH:mm:ss'), id])
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
    await initCronTask()
    await initAperiodicityTask()
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