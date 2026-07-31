// ============================================================
// 栖记 NestDiary - 日记文件操作（Markdown 文件存储）
// ============================================================

use std::fs;
use std::path::PathBuf;

/// 获取日记文件路径（按年/月组织）
pub fn get_diary_file_path(data_dir: &PathBuf, date: &str) -> PathBuf {
    // date 格式: YYYY-MM-DD
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() >= 2 {
        let year = parts[0];
        let month = parts[1];
        data_dir.join("diaries").join(year).join(month).join(format!("{}.md", date))
    } else {
        data_dir.join("diaries").join(format!("{}.md", date))
    }
}

/// 保存日记为 Markdown 文件
pub fn save_diary_file(data_dir: &PathBuf, date: &str, title: &str, content: &str) -> Result<(), String> {
    let file_path = get_diary_file_path(data_dir, date);

    // 确保目录存在
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 构建 Markdown 内容
    let markdown = if title.is_empty() {
        format!("> {}\n\n{}", date, content)
    } else {
        format!("# {}\n\n> {}\n\n{}", title, date, content)
    };

    fs::write(&file_path, markdown).map_err(|e| format!("写入日记文件失败: {}", e))
}

/// 读取日记 Markdown 文件
pub fn read_diary_file(data_dir: &PathBuf, date: &str) -> Result<Option<String>, String> {
    let file_path = get_diary_file_path(data_dir, date);

    if file_path.exists() {
        let content = fs::read_to_string(&file_path).map_err(|e| format!("读取日记文件失败: {}", e))?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

/// 删除日记文件
pub fn delete_diary_file(data_dir: &PathBuf, date: &str) -> Result<(), String> {
    let file_path = get_diary_file_path(data_dir, date);
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| format!("删除日记文件失败: {}", e))?;
    }
    Ok(())
}

/// 导出所有日记为 Markdown 文件
pub fn export_all_diaries(data_dir: &PathBuf, export_dir: &PathBuf) -> Result<u32, String> {
    let diaries_dir = data_dir.join("diaries");
    if !diaries_dir.exists() {
        return Ok(0);
    }

    let mut count = 0;

    // 递归遍历所有 .md 文件
    fn copy_md_files(src: &PathBuf, dest: &PathBuf, count: &mut u32) -> Result<(), String> {
        let entries = fs::read_dir(src).map_err(|e| format!("读取目录失败: {}", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                copy_md_files(&path, dest, count)?;
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Some(filename) = path.file_name() {
                    let dest_path = dest.join(filename);
                    fs::copy(&path, &dest_path).map_err(|e| format!("复制文件失败: {}", e))?;
                    *count += 1;
                }
            }
        }
        Ok(())
    }

    fs::create_dir_all(export_dir).map_err(|e| format!("创建导出目录失败: {}", e))?;
    copy_md_files(&diaries_dir, export_dir, &mut count)?;

    Ok(count)
}
