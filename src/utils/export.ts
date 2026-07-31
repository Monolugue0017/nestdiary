// ============================================================
// 导出与备份工具
// ============================================================

import JSZip from "jszip";
import { db } from "../db/database";
import { formatDate } from "./date";

/** 将 HTML 转换为 Markdown */
function htmlToMarkdown(html: string): string {
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;

  function convertNode(node: Node): string {
    if (node.nodeType === Node.TEXT_NODE) {
      return node.textContent || "";
    }
    if (node.nodeType !== Node.ELEMENT_NODE) return "";

    const el = node as HTMLElement;
    const tag = el.tagName.toLowerCase();
    const inner = Array.from(el.childNodes).map(convertNode).join("");

    switch (tag) {
      case "h1": return `\n# ${inner}\n\n`;
      case "h2": return `\n## ${inner}\n\n`;
      case "h3": return `\n### ${inner}\n\n`;
      case "p": return `${inner}\n\n`;
      case "strong":
      case "b": return `**${inner}**`;
      case "em":
      case "i": return `*${inner}*`;
      case "ul": return Array.from(el.children).map(li => `- ${convertNode(li)}\n`).join("");
      case "ol": return Array.from(el.children).map((li, i) => `${i + 1}. ${convertNode(li)}\n`).join("");
      case "li": return inner;
      case "blockquote": return `> ${inner}\n`;
      case "code": return `\`${inner}\``;
      case "pre": return `\n\`\`\`\n${el.textContent}\n\`\`\`\n\n`;
      case "br": return "\n";
      case "a": return `[${inner}](${el.getAttribute("href") || ""})`;
      default: return inner;
    }
  }

  return convertNode(tempDiv).trim();
}

/** 导出单篇日记为 Markdown 文件 */
export function exportDiaryAsMarkdown(title: string, date: string, content: string) {
  const markdown = `# ${title || date}\n\n> ${date}\n\n${htmlToMarkdown(content)}`;
  const blob = new Blob([markdown], { type: "text/markdown;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${date}.md`;
  a.click();
  URL.revokeObjectURL(url);
}

/** 导出所有日记为 Markdown 文件集合 */
export async function exportAllDiariesAsMarkdown() {
  const diaries = await db.diaries.orderBy("date").toArray();
  for (const diary of diaries) {
    exportDiaryAsMarkdown(diary.title, diary.date, diary.content);
  }
}

/** 一键备份所有数据为 ZIP 文件 */
export async function backupAllData() {
  const zip = new JSZip();

  // 导出日记为 Markdown 文件
  const diaries = await db.diaries.orderBy("date").toArray();
  const diaryFolder = zip.folder("diaries")!;
  for (const diary of diaries) {
    const md = `# ${diary.title || diary.date}\n\n> ${diary.date}\n\n${htmlToMarkdown(diary.content)}`;
    diaryFolder.file(`${diary.date}.md`, md);
  }

  // 导出待办为 JSON
  const todos = await db.todos.where("deleted").equals(0).toArray();
  zip.file("todos.json", JSON.stringify(todos, null, 2));

  // 导出备忘为 JSON
  const memos = await db.memos.toArray();
  zip.file("memos.json", JSON.stringify(memos, null, 2));

  // 导出设置（排除密码相关敏感信息）
  const settings = await db.settings.toArray();
  const safeSettings = settings.filter((s) => !s.key.startsWith("password_"));
  zip.file("settings.json", JSON.stringify(safeSettings, null, 2));

  // 生成备份信息文件
  const backupInfo = {
    app: "NestDiary",
    version: "0.1.0",
    exportDate: new Date().toISOString(),
    diaryCount: diaries.length,
    todoCount: todos.length,
    memoCount: memos.length,
  };
  zip.file("backup-info.json", JSON.stringify(backupInfo, null, 2));

  // 生成 ZIP 并下载
  const blob = await zip.generateAsync({ type: "blob" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `nestdiary-backup-${formatDate(new Date())}.zip`;
  a.click();
  URL.revokeObjectURL(url);
}

/** 从备份 ZIP 恢复数据 */
export async function restoreFromBackup(file: File): Promise<void> {
  const zip = await JSZip.loadAsync(file);

  // 恢复日记
  const diaryFolder = zip.folder("diaries");
  if (diaryFolder) {
    const files = Object.keys(zip.files).filter((path) => path.startsWith("diaries/") && path.endsWith(".md"));
    for (const path of files) {
      const content = await zip.files[path].async("string");
      const dateMatch = path.match(/diaries\/(\d{4}-\d{2}-\d{2})\.md/);
      if (dateMatch) {
        const date = dateMatch[1];
        const existing = await db.diaries.where("date").equals(date).first();
        if (!existing) {
          await db.diaries.add({
            date,
            title: "",
            content: content,
            plainText: content.replace(/[#*>\-`\[\]]/g, ""),
            wordCount: content.length,
            createdAt: Date.now(),
            updatedAt: Date.now(),
          });
        }
      }
    }
  }

  // 恢复待办
  const todosFile = zip.file("todos.json");
  if (todosFile) {
    const todos = JSON.parse(await todosFile.async("string"));
    for (const todo of todos) {
      delete todo.id;
      await db.todos.add(todo);
    }
  }

  // 恢复备忘
  const memosFile = zip.file("memos.json");
  if (memosFile) {
    const memos = JSON.parse(await memosFile.async("string"));
    for (const memo of memos) {
      delete memo.id;
      await db.memos.add(memo);
    }
  }
}
