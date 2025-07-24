// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod utils {
    pub mod network_util;
    pub mod random_util;
    pub mod scheduled_tasks;
    pub mod snowflake;
}

use std::sync::Arc;

use utils::random_util::{
    build_bank_info, build_id_card, build_name, build_phone, build_table_data,
};

pub use utils::scheduled_tasks::{
    cancel_cron_task, cancel_reminder, schedule_cron_task, schedule_reminder, send_notification,
};

use utils::network_util::is_port_open;
use utils::snowflake::generate_snowflake_id;

use tauri::{
    image::Image,
    menu::{IconMenuItem, Menu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

pub use utils::snowflake::Snowflake;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Snowflake::new(1, 2)))
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            build_phone,
            build_id_card,
            build_name,
            build_bank_info,
            build_table_data,
            generate_snowflake_id,
            is_port_open,
            schedule_reminder,
            send_notification,
            schedule_cron_task,
            cancel_cron_task,
            cancel_reminder
        ])
        .setup(|app| {
            tauri::async_runtime::spawn(async {
                use crate::utils::scheduled_tasks::SCHEDULER;

                if let Err(e) = SCHEDULER
                    .get_or_try_init(tokio_cron_scheduler::JobScheduler::new)
                    .await
                {
                    eprintln!("Scheduler init failed: {}", e);
                } else if let Err(e) = SCHEDULER.get().unwrap().start().await {
                    eprintln!("Scheduler start failed: {}", e);
                } else {
                    println!("Job scheduler started ✅");
                }
            });

            let exit_icon_image = Image::from_bytes(include_bytes!("../icons/exit.png")).unwrap();
            let show_hide_icon_image =
                Image::from_bytes(include_bytes!("../icons/ShowHide.png")).unwrap();
            let show_i = IconMenuItem::with_id(
                app,
                "show",
                "显示/隐藏",
                true,
                Some(show_hide_icon_image),
                None::<&str>,
            )
            .unwrap();
            let quit_i = IconMenuItem::with_id(
                app,
                "quit",
                &"退出程序",
                true,
                Some(exit_icon_image),
                None::<&str>,
            )
            .unwrap();
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            // 创建系统托盘
            let _tray = TrayIconBuilder::new()
                // 添加托盘图标
                .icon(Image::from_bytes(include_bytes!("../icons/ico.png")).expect("REASON"))
                // .icon(app.default_window_icon().unwrap().clone())
                // 添加菜单
                .menu(&menu)
                // 禁用鼠标左键点击图标显示托盘菜单
                .show_menu_on_left_click(false)
                // 监听托盘图标发出的鼠标事件
                .on_tray_icon_event(|tray, event| match event {
                    // 左键点击托盘图标显示窗口
                    TrayIconEvent::Click {
                        id: _,
                        position: _,
                        rect: _,
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                    } => {
                        let win = tray
                            .app_handle()
                            .get_webview_window("main")
                            .expect("REASON");
                        match win.is_visible() {
                            Ok(visible) => {
                                if !visible {
                                    win.show().unwrap();
                                    win.set_focus().unwrap();
                                } else {
                                    win.hide().unwrap();
                                }
                            }
                            Err(e) => eprintln!("{}", e),
                        };
                        // 获取窗口焦点
                        win.set_focus().unwrap();
                    }
                    _ => {}
                })
                // 监听菜单事件
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        let win = app.get_webview_window("main").unwrap();
                        match win.is_visible() {
                            Ok(visible) => {
                                if !visible {
                                    win.show().unwrap();
                                } else {
                                    win.hide().unwrap();
                                }
                            }
                            Err(e) => eprintln!("{}", e),
                        };
                        // 获取窗口焦点
                        win.set_focus().unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
