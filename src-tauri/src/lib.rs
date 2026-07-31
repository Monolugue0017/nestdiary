// ============================================================
// 栖记 NestDiary - Tauri 应用入口
// ============================================================

mod commands;
mod db;
mod diary;

use db::init_database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化数据库
            let db_state = init_database(app.handle())?;
            app.manage(db_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 日记命令
            commands::get_diary_by_date,
            commands::save_diary,
            commands::get_all_diaries,
            commands::delete_diary,
            // 待办命令
            commands::get_todos,
            commands::add_todo,
            commands::update_todo,
            commands::delete_todo,
            // 备忘命令
            commands::get_memos,
            commands::add_memo,
            commands::update_memo,
            commands::delete_memo,
            // 设置命令
            commands::get_setting,
            commands::set_setting,
            // 搜索命令
            commands::search_all,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
