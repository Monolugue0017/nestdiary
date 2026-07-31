// ============================================================
// 栖记 NestDiary - Tauri 命令定义
// 前端通过 invoke() 调用这些命令
// ============================================================

use crate::db::{DbState, get_data_dir};
use crate::diary;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

// ============================================================
// 数据模型
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Diary {
    pub id: Option<i64>,
    pub date: String,
    pub title: String,
    pub content: String,
    pub plain_text: String,
    pub word_count: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<i64>,
    pub text: String,
    pub completed: bool,
    pub priority: i64,
    pub deleted: bool,
    pub completed_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memo {
    pub id: Option<i64>,
    pub content: String,
    pub pinned: bool,
    pub color: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub result_type: String,
    pub id: i64,
    pub title: String,
    pub snippet: String,
    pub date: String,
    pub updated_at: i64,
}

// ============================================================
// 日记命令
// ============================================================

#[tauri::command]
pub fn get_diary_by_date(date: String, state: State<DbState>) -> Result<Option<Diary>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, date, title, content, plain_text, word_count, created_at, updated_at FROM diaries WHERE date = ?1")
        .map_err(|e| e.to_string())?;

    let diary = stmt
        .query_row(params![date], |row| {
            Ok(Diary {
                id: Some(row.get(0)?),
                date: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                plain_text: row.get(4)?,
                word_count: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .ok();

    Ok(diary)
}

#[tauri::command]
pub fn save_diary(diary: Diary, app: AppHandle, state: State<DbState>) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp_millis();

    // 同时保存到 SQLite 和 Markdown 文件
    let data_dir = get_data_dir(&app)?;

    let id = if let Some(existing_id) = diary.id {
        // 更新
        conn.execute(
            "UPDATE diaries SET title = ?1, content = ?2, plain_text = ?3, word_count = ?4, updated_at = ?5 WHERE id = ?6",
            params![diary.title, diary.content, diary.plain_text, diary.word_count, now, existing_id],
        )
        .map_err(|e| e.to_string())?;
        existing_id
    } else {
        // 插入
        conn.execute(
            "INSERT INTO diaries (date, title, content, plain_text, word_count, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![diary.date, diary.title, diary.content, diary.plain_text, diary.word_count, now, now],
        )
        .map_err(|e| e.to_string())?;
        conn.last_insert_rowid()
    };

    // 保存 Markdown 文件
    diary::save_diary_file(&data_dir, &diary.date, &diary.title, &diary.plain_text)?;

    Ok(id)
}

#[tauri::command]
pub fn get_all_diaries(state: State<DbState>) -> Result<Vec<Diary>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, date, title, content, plain_text, word_count, created_at, updated_at FROM diaries ORDER BY date DESC")
        .map_err(|e| e.to_string())?;

    let diaries = stmt
        .query_map([], |row| {
            Ok(Diary {
                id: Some(row.get(0)?),
                date: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                plain_text: row.get(4)?,
                word_count: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(diaries)
}

#[tauri::command]
pub fn delete_diary(id: i64, date: String, app: AppHandle, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM diaries WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    let data_dir = get_data_dir(&app)?;
    diary::delete_diary_file(&data_dir, &date)?;

    Ok(())
}

// ============================================================
// 待办命令
// ============================================================

#[tauri::command]
pub fn get_todos(state: State<DbState>) -> Result<Vec<Todo>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, text, completed, priority, deleted, completed_at, created_at, updated_at FROM todos WHERE deleted = 0 ORDER BY completed ASC, priority ASC, created_at DESC")
        .map_err(|e| e.to_string())?;

    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: Some(row.get(0)?),
                text: row.get(1)?,
                completed: row.get(2)?,
                priority: row.get(3)?,
                deleted: row.get(4)?,
                completed_at: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(todos)
}

#[tauri::command]
pub fn add_todo(text: String, priority: i64, state: State<DbState>) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp_millis();

    conn.execute(
        "INSERT INTO todos (text, completed, priority, deleted, created_at, updated_at) VALUES (?1, 0, ?2, 0, ?3, ?4)",
        params![text, priority, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn update_todo(id: i64, text: Option<String>, completed: Option<bool>, priority: Option<i64>, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp_millis();

    if let Some(text) = text {
        conn.execute("UPDATE todos SET text = ?1, updated_at = ?2 WHERE id = ?3", params![text, now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(completed) = completed {
        let completed_at = if completed { Some(now) } else { None };
        conn.execute("UPDATE todos SET completed = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
            params![completed as i64, completed_at, now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(priority) = priority {
        conn.execute("UPDATE todos SET priority = ?1, updated_at = ?2 WHERE id = ?3", params![priority, now, id])
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn delete_todo(id: i64, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp_millis();
    conn.execute("UPDATE todos SET deleted = 1, updated_at = ?1 WHERE id = ?2", params![now, id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================
// 备忘命令
// ============================================================

#[tauri::command]
pub fn get_memos(state: State<DbState>) -> Result<Vec<Memo>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, content, pinned, color, created_at, updated_at FROM memos ORDER BY pinned DESC, updated_at DESC")
        .map_err(|e| e.to_string())?;

    let memos = stmt
        .query_map([], |row| {
            Ok(Memo {
                id: Some(row.get(0)?),
                content: row.get(1)?,
                pinned: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(memos)
}

#[tauri::command]
pub fn add_memo(content: String, pinned: bool, state: State<DbState>) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp_millis();

    conn.execute(
        "INSERT INTO memos (content, pinned, color, created_at, updated_at) VALUES (?1, ?2, '', ?3, ?4)",
        params![content, pinned as i64, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn update_memo(id: i64, content: Option<String>, pinned: Option<bool>, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp_millis();

    if let Some(content) = content {
        conn.execute("UPDATE memos SET content = ?1, updated_at = ?2 WHERE id = ?3", params![content, now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(pinned) = pinned {
        conn.execute("UPDATE memos SET pinned = ?1, updated_at = ?2 WHERE id = ?3", params![pinned as i64, now, id])
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn delete_memo(id: i64, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM memos WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================
// 设置命令
// ============================================================

#[tauri::command]
pub fn get_setting(key: String, state: State<DbState>) -> Result<Option<String>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let result = conn
        .query_row("SELECT value FROM settings WHERE key = ?1", params![key], |row| {
            row.get::<_, Option<String>>(0)
        })
        .ok()
        .flatten();
    Ok(result)
}

#[tauri::command]
pub fn set_setting(key: String, value: String, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================
// 搜索命令
// ============================================================

#[tauri::command]
pub fn search_all(keyword: String, state: State<DbState>) -> Result<Vec<SearchResult>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    let lower_keyword = keyword.to_lowercase();

    // 搜索日记（使用 FTS5）
    let mut stmt = conn
        .prepare("SELECT id, date, title, plain_text, updated_at FROM diaries WHERE plain_text LIKE ?1 OR title LIKE ?2 ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let pattern = format!("%{}%", lower_keyword);
    let diary_results = stmt
        .query_map(params![pattern, pattern], |row| {
            let id: i64 = row.get(0)?;
            let date: String = row.get(1)?;
            let title: String = row.get(2)?;
            let plain_text: String = row.get(3)?;
            let updated_at: i64 = row.get(4)?;

            let snippet = extract_snippet(&plain_text, &lower_keyword);
            Ok(SearchResult {
                result_type: "diary".to_string(),
                id,
                title: if title.is_empty() { date.clone() } else { title },
                snippet,
                date,
                updated_at,
            })
        })
        .map_err(|e| e.to_string())?;

    for result in diary_results {
        results.push(result.map_err(|e| e.to_string())?);
    }

    // 搜索待办
    let mut stmt = conn
        .prepare("SELECT id, text, created_at, updated_at FROM todos WHERE deleted = 0 AND text LIKE ?1 ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let todo_results = stmt
        .query_map(params![pattern], |row| {
            let id: i64 = row.get(0)?;
            let text: String = row.get(1)?;
            let created_at: i64 = row.get(2)?;
            let updated_at: i64 = row.get(3)?;

            let snippet = extract_snippet(&text, &lower_keyword);
            let date = chrono::DateTime::from_timestamp_millis(created_at)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            Ok(SearchResult {
                result_type: "todo".to_string(),
                id,
                title: text.chars().take(40).collect(),
                snippet,
                date,
                updated_at,
            })
        })
        .map_err(|e| e.to_string())?;

    for result in todo_results {
        results.push(result.map_err(|e| e.to_string())?);
    }

    // 搜索备忘
    let mut stmt = conn
        .prepare("SELECT id, content, created_at, updated_at FROM memos WHERE content LIKE ?1 ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let memo_results = stmt
        .query_map(params![pattern], |row| {
            let id: i64 = row.get(0)?;
            let content: String = row.get(1)?;
            let created_at: i64 = row.get(2)?;
            let updated_at: i64 = row.get(3)?;

            let snippet = extract_snippet(&content, &lower_keyword);
            let date = chrono::DateTime::from_timestamp_millis(created_at)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            Ok(SearchResult {
                result_type: "memo".to_string(),
                id,
                title: content.chars().take(40).collect(),
                snippet,
                date,
                updated_at,
            })
        })
        .map_err(|e| e.to_string())?;

    for result in memo_results {
        results.push(result.map_err(|e| e.to_string())?);
    }

    Ok(results)
}

/// 提取关键词周围的文本片段
fn extract_snippet(text: &str, keyword: &str, radius: usize) -> String {
    let lower_text = text.to_lowercase();
    let index = match lower_text.find(keyword) {
        Some(i) => i,
        None => return text.chars().take(100).collect(),
    };

    let start = if index > radius { index - radius } else { 0 };
    let end = (index + keyword.len() + radius).min(text.len());

    let prefix = if start > 0 { "..." } else { "" };
    let suffix = if end < text.len() { "..." } else { "" };

    format!("{}{}{}", prefix, &text[start..end], suffix)
}
