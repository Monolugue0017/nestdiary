// ============================================================
// 栖记 NestDiary - 类型定义
// ============================================================

/** 日记条目 */
export interface Diary {
  /** 主键，自增 */
  id?: number;
  /** 日记日期，格式 YYYY-MM-DD */
  date: string;
  /** 日记标题（可选） */
  title: string;
  /** 日记正文，HTML 格式（Tiptap 输出） */
  content: string;
  /** 纯文本内容（用于搜索） */
  plainText: string;
  /** 字数统计 */
  wordCount: number;
  /** 创建时间戳 */
  createdAt: number;
  /** 更新时间戳 */
  updatedAt: number;
}

/** 待办事项 */
export interface Todo {
  id?: number;
  /** 待办内容 */
  text: string;
  /** 是否完成 */
  completed: boolean;
  /** 优先级：1=高 2=中 3=低 */
  priority: 1 | 2 | 3;
  /** 是否删除（软删除） */
  deleted: boolean;
  /** 完成时间戳 */
  completedAt?: number;
  createdAt: number;
  updatedAt: number;
}

/** 备忘录 */
export interface Memo {
  id?: number;
  /** 备忘内容 */
  content: string;
  /** 是否置顶 */
  pinned: boolean;
  /** 颜色标签 */
  color?: string;
  createdAt: number;
  updatedAt: number;
}

/** 应用设置 */
export interface AppSetting {
  key: string;
  value: any;
}

/** 搜索结果 */
export interface SearchResult {
  type: "diary" | "todo" | "memo";
  id: number;
  title: string;
  snippet: string;
  date: string;
  updatedAt: number;
}

/** 模块类型 */
export type ModuleType = "diary" | "todo" | "memo";
