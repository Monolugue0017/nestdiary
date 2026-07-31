// ============================================================
// 日期工具函数
// ============================================================

/** 格式化日期为 YYYY-MM-DD */
export function formatDate(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

/** 格式化日期为中文显示 */
export function formatDateChinese(date: Date): string {
  const weekdays = ["日", "一", "二", "三", "四", "五", "六"];
  return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日 星期${weekdays[date.getDay()]}`;
}

/** 从 YYYY-MM-DD 字符串解析为 Date 对象 */
export function parseDate(dateStr: string): Date {
  const [year, month, day] = dateStr.split("-").map(Number);
  return new Date(year, month - 1, day);
}

/** 获取今天的日期字符串 */
export function today(): string {
  return formatDate(new Date());
}

/** 获取指定日期前/后几天的日期字符串 */
export function addDays(dateStr: string, days: number): string {
  const date = parseDate(dateStr);
  date.setDate(date.getDate() + days);
  return formatDate(date);
}

/** 判断是否是今天 */
export function isToday(dateStr: string): boolean {
  return dateStr === today();
}

/** 格式化时间戳为显示文字 */
export function formatTimestamp(ts: number): string {
  const date = new Date(ts);
  const now = new Date();
  const diff = now.getTime() - ts;

  if (diff < 60000) return "刚刚";
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`;
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}小时前`;
  if (diff < 604800000) return `${Math.floor(diff / 86400000)}天前`;

  return formatDate(date);
}

/** 获取两个日期之间的天数差 */
export function daysBetween(dateStr1: string, dateStr2: string): number {
  const d1 = parseDate(dateStr1);
  const d2 = parseDate(dateStr2);
  return Math.round((d2.getTime() - d1.getTime()) / 86400000);
}
