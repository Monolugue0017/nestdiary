import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { getSetting, setSetting } from "../db/database";
import { verifyPassword, hashPassword, generateSalt } from "../utils/crypto";

export const useAppStore = defineStore("app", () => {
  // ============================================================
  // 密码锁状态
  // ============================================================

  /** 是否已设置密码 */
  const hasPassword = ref(false);
  /** 是否已解锁 */
  const isUnlocked = ref(false);
  /** 是否正在锁定屏 */
  const isLocked = ref(false);
  /** 自动锁定时间（分钟），0 表示不自动锁定 */
  const autoLockMinutes = ref(5);
  /** 最后活动时间 */
  const lastActivityTime = ref(Date.now());

  // ============================================================
  // 主题状态
  // ============================================================

  /** 当前主题：light / dark */
  const theme = ref<"light" | "dark">("light");

  // ============================================================
  // 初始化
  // ============================================================

  async function init() {
    // 加载密码设置
    const passwordHash = await getSetting("password_hash");
    hasPassword.value = !!passwordHash;

    // 如果没有密码，直接解锁
    if (!hasPassword.value) {
      isUnlocked.value = true;
    }

    // 加载自动锁定时间
    const autoLock = await getSetting("auto_lock_minutes");
    if (autoLock !== undefined) {
      autoLockMinutes.value = autoLock;
    }

    // 加载主题
    const savedTheme = await getSetting("theme");
    if (savedTheme) {
      theme.value = savedTheme;
    }

    // 启动自动锁定检测
    startAutoLockCheck();
  }

  // ============================================================
  // 密码锁操作
  // ============================================================

  /** 设置密码 */
  async function setPassword(password: string) {
    const salt = generateSalt();
    const hash = await hashPassword(password, salt);
    await setSetting("password_hash", hash);
    await setSetting("password_salt", salt);
    hasPassword.value = true;
    isUnlocked.value = true;
    isLocked.value = false;
  }

  /** 验证密码 */
  async function unlock(password: string): Promise<boolean> {
    const hash = await getSetting("password_hash");
    const salt = await getSetting("password_salt");
    if (!hash || !salt) return true;

    const valid = await verifyPassword(password, salt, hash);
    if (valid) {
      isUnlocked.value = true;
      isLocked.value = false;
      lastActivityTime.value = Date.now();
    }
    return valid;
  }

  /** 锁定应用 */
  function lock() {
    isLocked.value = true;
    isUnlocked.value = false;
  }

  /** 关闭密码锁 */
  async function removePassword() {
    await setSetting("password_hash", null);
    await setSetting("password_salt", null);
    hasPassword.value = false;
    isUnlocked.value = true;
    isLocked.value = false;
  }

  /** 设置自动锁定时间 */
  async function setAutoLockMinutes(minutes: number) {
    autoLockMinutes.value = minutes;
    await setSetting("auto_lock_minutes", minutes);
  }

  /** 更新活动时间 */
  function updateActivity() {
    lastActivityTime.value = Date.now();
  }

  /** 自动锁定检测循环 */
  function startAutoLockCheck() {
    setInterval(() => {
      if (!hasPassword.value || !isUnlocked.value || autoLockMinutes.value === 0) return;
      const elapsed = Date.now() - lastActivityTime.value;
      if (elapsed >= autoLockMinutes.value * 60 * 1000) {
        lock();
      }
    }, 10000); // 每 10 秒检查一次
  }

  // ============================================================
  // 主题操作
  // ============================================================

  async function toggleTheme() {
    theme.value = theme.value === "light" ? "dark" : "light";
    await setSetting("theme", theme.value);
  }

  return {
    // 状态
    hasPassword,
    isUnlocked,
    isLocked,
    autoLockMinutes,
    theme,
    // 初始化
    init,
    // 密码锁
    setPassword,
    unlock,
    lock,
    removePassword,
    setAutoLockMinutes,
    updateActivity,
    // 主题
    toggleTheme,
  };
});
