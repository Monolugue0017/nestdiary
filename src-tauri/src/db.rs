// ============================================================
// 栖记 NestDiary - SQLite 数据库管理
// ============================================================

use rusqlite::{Connection, params};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

/// 数据库状态，通过 Tauri 状态管理共享
pub struct DbState(pub Mutex<Connection>);

/// 初始化数据库
pub fn init_database(app: &AppHandle) -> Result<DbState, String> {
    // 获取应用数据目录
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;

    // 确保目录存在
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("创建数据目录失败: {}", e))?;

    let db_path = app_dir.join("nestdiary.db");

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;

    // 创建日记目录
    let diary_dir = app_dir.join("diaries");
    std::fs::create_dir_all(&diary_dir)
        .map_err(|e| format!("创建日记目录失败: {}", e))?;

    // 创建表
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS diaries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL UNIQUE,
            title TEXT DEFAULT '',
            content TEXT DEFAULT '',
            plain_text TEXT DEFAULT '',
            word_count INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            priority INTEGER DEFAULT 2,
            deleted INTEGER DEFAULT 0,
            completed_at INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS memos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            pinned INTEGER DEFAULT 0,
            color TEXT DEFAULT '',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );

        -- 全文搜索虚拟表（FTS5）
        CREATE VIRTUAL TABLE IF NOT EXISTS diaries_fts USING fts5(
            date, title, plain_text,
            content='diaries',
            content_rowid='id'
        );

        -- 触发器：自动同步 FTS 索引
        CREATE TRIGGER IF NOT EXISTS diaries_ai AFTER INSERT ON diaries BEGIN
            INSERT INTO diaries_fts(rowid, date, title, plain_text)
            VALUES (new.id, new.date, new.title, new.plain_text);
        END;

        CREATE TRIGGER IF NOT EXISTS diaries_ad AFTER DELETE ON diaries BEGIN
            INSERT INTO diaries_fts(diaries_fts, rowid, date, title, plain_text)
            VALUES ('delete', old.id, old.date, old.title, old.plain_text);
        END;

        CREATE TRIGGER IF NOT EXISTS diaries_au AFTER UPDATE ON diaries BEGIN
            INSERT INTO diaries_fts(diaries_fts, rowid, date, title, plain_text)
            VALUES ('delete', old.id, old.date, old.title, old.plain_text);
            INSERT INTO diaries_fts(rowid, date, title, plain_text)
            VALUES (new.id, new.date, new.title, new.plain_text);
        END;
        ",
    )
    .map_err(|e| format!("初始化数据库表失败: {}", e))?;

    Ok(DbState(Mutex::new(conn)))
}

/// 获取应用数据目录路径
pub fn get_data_dir(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))
}
