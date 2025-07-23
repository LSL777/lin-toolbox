use chrono::{Local, NaiveDateTime, TimeZone};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::OnceCell;
use tokio::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

/// 一次性延迟任务对象
#[derive(Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub content: String,
    /// 格式 "YYYY-MM-DD HH:mm:ss"
    pub remind_time: String,
}

/// cron任务对象
#[derive(Deserialize)]
pub struct CronTask {
    pub id: String,
    pub content: String,
    pub cron_expr: String,
}

/// 全局任务池，Key 是 CronTask.id，Value 是 Job UUID
pub static TASK_POOL: Lazy<Mutex<HashMap<String, uuid::Uuid>>> = Lazy::new(|| Mutex::new(HashMap::new()));
/// 全局任务调度器
pub static SCHEDULER: OnceCell<JobScheduler> = OnceCell::const_new();

/// 创建一次性延迟任务命令
#[tauri::command]
pub async fn schedule_reminder(app_handle: AppHandle, todo: Todo) -> Result<(), String> {
    let format = "%Y-%m-%d %H:%M:%S";

    // 解析提醒时间
    let remind_time = NaiveDateTime::parse_from_str(&todo.remind_time, format)
        .map_err(|e| format!("Failed to parse date time: {}", e))?;

    // 启动定时任务
    tokio::spawn(async move {
        schedule_one_time_reminder(app_handle, todo.id, remind_time, todo.content).await;
    });

    Ok(())
}

/// 发送通知
/// - task_type: 0: 非周期性任务 1: 周期性任务
#[tauri::command]
pub fn send_notification(app_handle: AppHandle, todo_id: String, content: String, task_type: i8) {
    println!("send_notification_for_tasks: {}", todo_id);
    app_handle
        .notification()
        .builder()
        .title("待办事件通知")
        .body(content)
        .show()
        .unwrap();
    let event_data = serde_json::json!({
        "todo_id": todo_id,
        "task_type": task_type
    });

    // 向前端发送事件
    app_handle
        .emit_to("main", "reminder_triggered", event_data)
        .unwrap();
}


/// 取消cron表达式任务
#[tauri::command]
pub async fn cancel_cron_task(id: String) -> Result<String, String> {
    // 提前移除任务 ID，避免跨 .await 锁住 Mutex
    let job_uuid = {
        let mut pool = TASK_POOL.lock().unwrap();
        pool.remove(&id).ok_or_else(|| format!("No task found with id: {}", id))?
    };

    // 获取 scheduler
    let scheduler = SCHEDULER
        .get()
        .ok_or_else(|| "Scheduler not initialized".to_string())?;

    println!("Canceled: {}", &job_uuid);
    scheduler
        .remove(&job_uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Task {} canceled", id))
}



/// 创建cron表达式定时任务
#[tauri::command]
pub async fn schedule_cron_task(app_handle: AppHandle, task: CronTask) -> Result<String, String> {
    let task_id = task.id.clone();

    let scheduler = SCHEDULER
        .get()
        .ok_or_else(|| "Scheduler not initialized".to_string())?;

    let job = Job::new_async(&task.cron_expr, move |_uuid, _l| {
        let app_handle = app_handle.clone();
        let content = task.content.clone();
        let id = task.id.clone();
        Box::pin(async move {
            let trigger_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("周期任务触发：触发事件 {}, 任务ID {}, 内容 '{}'", trigger_time, id, content);
            send_notification(app_handle, id, content, 1);
        })
    }).map_err(|e| format!("解析 cron 表达式失败: {}", e))?;

    let job_id = scheduler
        .add(job)
        .await
        .map_err(|e| e.to_string())?;

    TASK_POOL
        .lock()
        .unwrap()
        .insert(task_id.clone(), job_id);

    println!("task_id: {}, job_id: {}", task_id, job_id);
    Ok(format!("任务已创建，任务ID: {}", job_id))
}


/// 创建一次性延迟任务
async fn schedule_one_time_reminder(
    app_handle: AppHandle,
    todo_id: String,
    remind_time: NaiveDateTime,
    content: String,
) {
    let remind_time_local = Local.from_local_datetime(&remind_time).single();

    match remind_time_local {
        Some(remind_time_local) => {
            let now = Local::now(); // 使用本地时间
            let delay = (remind_time_local - now)
                .to_std()
                .unwrap_or(Duration::from_secs(0));

            tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                send_notification(app_handle, todo_id, content, 0);
            });
        }
        None => {
            eprintln!("提醒时间解析失败，可能是无效的本地时间");
        }
    }
}
