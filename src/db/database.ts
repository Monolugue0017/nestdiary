// ============================================================
// 栖记 NestDiary - 数据存储层
// 基于 Dexie.js (IndexedDB) 实现纯本地存储
// 桌面版可替换为 Tauri + Rust + SQLite 后端
// ============================================================

import Dexie, { type Table } from "dexie";
import type { Diary, Todo, Memo, AppSetting } from "../types";

export class NestDiaryDB extends Dexie {
  diaries!: Table<Diary, number>;
  todos!: Table<Todo, number>;
  memos!: Table<Memo, number>;
  settings!: Table<AppSetting, string>;

  constructor() {
    super("NestDiaryDB");
    this.version(1).stores({
      // 主键 id，date 字段建索引用于按日期查询
      diaries: "++id, date, updatedAt",
      // 主键 id，completed/priority/deleted 建索引
      todos: "++id, completed, priority, deleted, updatedAt",
      // 主键 id，pinned 建索引
      memos: "++id, pinned, updatedAt",
      // key-value 存储
      settings: "key",
    });
  }
}

// 全局数据库实例
export const db = new NestDiaryDB();

// ============================================================
// 日记数据操作
// ============================================================

/** 根据日期获取日记 */
export async function getDiaryByDate(date: string): Promise<Diary | undefined> {
  return db.diaries.where("date").equals(date).first();
}

/** 获取所有日记（按日期倒序） */
export async function getAllDiaries(): Promise<Diary[]> {
  return db.diaries.orderBy("date").reverse().toArray();
}

/** 创建或更新日记 */
export async function saveDiary(diary: Diary): Promise<number> {
  if (diary.id) {
    await db.diaries.update(diary.id, { ...diary, updatedAt: Date.now() });
    return diary.id;
  }
  return db.diaries.add({ ...diary, createdAt: Date.now(), updatedAt: Date.now() });
}

/** 获取有日记的日期集合 */
export async function getDiaryDates(): Promise<string[]> {
  const all = await db.diaries.orderBy("date").toArray();
  return all.map((d) => d.date);
}

/** 删除日记 */
export async function deleteDiary(id: number): Promise<void> {
  await db.diaries.delete(id);
}

// ============================================================
// 待办数据操作
// ============================================================

/** 获取所有未删除的待办（按优先级+创建时间排序） */
export async function getTodos(): Promise<Todo[]> {
  const all = await db.todos.where("deleted").equals(0).toArray();
  // 排序：未完成在前，按优先级(1>2>3)，再按创建时间
  return all.sort((a, b) => {
    if (a.completed !== b.completed) return a.completed ? 1 : -1;
    if (a.priority !== b.priority) return a.priority - b.priority;
    return b.createdAt - a.createdAt;
  });
}

/** 创建待办 */
export async function addTodo(text: string, priority: 1 | 2 | 3 = 2): Promise<number> {
  const now = Date.now();
  return db.todos.add({
    text,
    completed: false,
    priority,
    deleted: false,
    createdAt: now,
    updatedAt: now,
  });
}

/** 更新待办 */
export async function updateTodo(id: number, changes: Partial<Todo>): Promise<void> {
  await db.todos.update(id, { ...changes, updatedAt: Date.now() });
}

/** 删除待办（软删除） */
export async function deleteTodo(id: number): Promise<void> {
  await db.todos.update(id, { deleted: true, updatedAt: Date.now() });
}

// ============================================================
// 备忘数据操作
// ============================================================

/** 获取所有备忘（置顶在前，再按更新时间倒序） */
export async function getMemos(): Promise<Memo[]> {
  const all = await db.memos.toArray();
  return all.sort((a, b) => {
    if (a.pinned !== b.pinned) return a.pinned ? -1 : 1;
    return b.updatedAt - a.updatedAt;
  });
}

/** 创建备忘 */
export async function addMemo(content: string, pinned = false): Promise<number> {
  const now = Date.now();
  return db.memos.add({
    content,
    pinned,
    createdAt: now,
    updatedAt: now,
  });
}

/** 更新备忘 */
export async function updateMemo(id: number, changes: Partial<Memo>): Promise<void> {
  await db.memos.update(id, { ...changes, updatedAt: Date.now() });
}

/** 删除备忘 */
export async function deleteMemo(id: number): Promise<void> {
  await db.memos.delete(id);
}

// ============================================================
// 设置数据操作
// ============================================================

/** 获取设置项 */
export async function getSetting(key: string): Promise<any> {
  const item = await db.settings.get(key);
  return item?.value;
}

/** 设置项 */
export async function setSetting(key: string, value: any): Promise<void> {
  await db.settings.put({ key, value });
}

/** 获取所有设置 */
export async function getAllSettings(): Promise<Record<string, any>> {
  const all = await db.settings.toArray();
  const result: Record<string, any> = {};
  for (const item of all) {
    result[item.key] = item.value;
  }
  return result;
}

// ============================================================
// 全文搜索
// ============================================================

import type { SearchResult } from "../types";

/** 跨日记+备忘+待办全文搜索 */
export async function searchAll(keyword: string): Promise<SearchResult[]> {
  if (!keyword.trim()) return [];
  const lowerKeyword = keyword.toLowerCase();
  const results: SearchResult[] = [];

  // 搜索日记
  const diaries = await db.diaries.toArray();
  for (const diary of diaries) {
    const inTitle = diary.title?.toLowerCase().includes(lowerKeyword);
    const inContent = diary.plainText?.toLowerCase().includes(lowerKeyword);
    if (inTitle || inContent) {
      const snippet = extractSnippet(diary.plainText || "", lowerKeyword);
      results.push({
        type: "diary",
        id: diary.id!,
        title: diary.title || diary.date,
        snippet,
        date: diary.date,
        updatedAt: diary.updatedAt,
      });
    }
  }

  // 搜索待办
  const todos = await db.todos.where("deleted").equals(0).toArray();
  for (const todo of todos) {
    if (todo.text.toLowerCase().includes(lowerKeyword)) {
      results.push({
        type: "todo",
        id: todo.id!,
        title: todo.text.slice(0, 40),
        snippet: extractSnippet(todo.text, lowerKeyword),
        date: new Date(todo.createdAt).toISOString().slice(0, 10),
        updatedAt: todo.updatedAt,
      });
    }
  }

  // 搜索备忘
  const memos = await db.memos.toArray();
  for (const memo of memos) {
    if (memo.content.toLowerCase().includes(lowerKeyword)) {
      results.push({
        type: "memo",
        id: memo.id!,
        title: memo.content.slice(0, 40),
        snippet: extractSnippet(memo.content, lowerKeyword),
        date: new Date(memo.createdAt).toISOString().slice(0, 10),
        updatedAt: memo.updatedAt,
      });
    }
  }

  // 按更新时间倒序
  return results.sort((a, b) => b.updatedAt - a.updatedAt);
}

/** 提取关键词周围的文本片段 */
function extractSnippet(text: string, keyword: string, radius = 50): string {
  const lowerText = text.toLowerCase();
  const index = lowerText.indexOf(keyword);
  if (index === -1) return text.slice(0, 100);
  const start = Math.max(0, index - radius);
  const end = Math.min(text.length, index + keyword.length + radius);
  const prefix = start > 0 ? "..." : "";
  const suffix = end < text.length ? "..." : "";
  return prefix + text.slice(start, end) + suffix;
}
