<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useAppStore } from "../stores/app";
import { backupAllData, exportAllDiariesAsMarkdown, restoreFromBackup } from "../utils/export";
import { db } from "../db/database";

const appStore = useAppStore();

// 密码锁
const showPasswordModal = ref(false);
const passwordMode = ref<"set" | "change" | "remove">("set");
const oldPassword = ref("");
const newPassword = ref("");
const confirmPassword = ref("");
const passwordError = ref("");

// 自动锁定
const autoLockOptions = [
  { value: 0, label: "不自动锁定" },
  { value: 1, label: "1 分钟" },
  { value: 5, label: "5 分钟" },
  { value: 10, label: "10 分钟" },
  { value: 30, label: "30 分钟" },
];

// 数据统计
const stats = ref({ diaries: 0, todos: 0, memos: 0 });

async function loadStats() {
  stats.value = {
    diaries: await db.diaries.count(),
    todos: await db.todos.where("deleted").equals(0).count(),
    memos: await db.memos.count(),
  };
}

// 密码操作
function openPasswordModal(mode: "set" | "change" | "remove") {
  passwordMode.value = mode;
  oldPassword.value = "";
  newPassword.value = "";
  confirmPassword.value = "";
  passwordError.value = "";
  showPasswordModal.value = true;
}

async function handlePasswordSubmit() {
  passwordError.value = "";

  if (passwordMode.value === "set") {
    if (!newPassword.value) {
      passwordError.value = "请输入密码";
      return;
    }
    if (newPassword.value.length < 4) {
      passwordError.value = "密码至少 4 位";
      return;
    }
    if (newPassword.value !== confirmPassword.value) {
      passwordError.value = "两次密码不一致";
      return;
    }
    await appStore.setPassword(newPassword.value);
    showPasswordModal.value = false;
  } else if (passwordMode.value === "change") {
    const valid = await appStore.unlock(oldPassword.value);
    if (!valid) {
      passwordError.value = "原密码错误";
      return;
    }
    if (newPassword.value.length < 4) {
      passwordError.value = "新密码至少 4 位";
      return;
    }
    if (newPassword.value !== confirmPassword.value) {
      passwordError.value = "两次密码不一致";
      return;
    }
    await appStore.setPassword(newPassword.value);
    showPasswordModal.value = false;
  } else if (passwordMode.value === "remove") {
    const valid = await appStore.unlock(oldPassword.value);
    if (!valid) {
      passwordError.value = "密码错误";
      return;
    }
    await appStore.removePassword();
    showPasswordModal.value = false;
  }
}

// 备份与导出
const isBackingUp = ref(false);
const backupStatus = ref("");

async function handleBackup() {
  isBackingUp.value = true;
  backupStatus.value = "正在打包备份...";
  try {
    await backupAllData();
    backupStatus.value = "备份成功！";
  } catch (e) {
    backupStatus.value = "备份失败：" + (e as Error).message;
  }
  isBackingUp.value = false;
  setTimeout(() => (backupStatus.value = ""), 3000);
}

async function handleExportAll() {
  await exportAllDiariesAsMarkdown();
}

async function handleRestore(event: Event) {
  const input = event.target as HTMLInputElement;
  if (!input.files || !input.files[0]) return;
  if (!confirm("恢复数据将覆盖现有数据，确定继续吗？")) return;
  try {
    await restoreFromBackup(input.files[0]);
    alert("数据恢复成功！");
    await loadStats();
  } catch (e) {
    alert("恢复失败：" + (e as Error).message);
  }
  input.value = "";
}

onMounted(loadStats);
</script>

<template>
  <div class="settings-view">
    <div class="settings-content">
      <!-- 密码锁设置 -->
      <section class="settings-section">
        <h2 class="section-title">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
            <path d="M7 11V7a5 5 0 0 1 10 0v4" />
          </svg>
          应用密码锁
        </h2>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">密码锁状态</span>
            <span class="setting-desc">{{ appStore.hasPassword ? "已启用" : "未启用" }}</span>
          </div>
          <div class="setting-actions">
            <button v-if="!appStore.hasPassword" class="action-btn primary" @click="openPasswordModal('set')">
              设置密码
            </button>
            <button v-else class="action-btn" @click="openPasswordModal('change')">
              修改密码
            </button>
            <button v-if="appStore.hasPassword" class="action-btn danger" @click="openPasswordModal('remove')">
              关闭密码锁
            </button>
          </div>
        </div>

        <div class="setting-item" v-if="appStore.hasPassword">
          <div class="setting-info">
            <span class="setting-label">自动锁定</span>
            <span class="setting-desc">空闲一定时间后自动锁定应用</span>
          </div>
          <select
            :value="appStore.autoLockMinutes"
            class="select-input"
            @change="appStore.setAutoLockMinutes(Number(($event.target as HTMLSelectElement).value))"
          >
            <option v-for="opt in autoLockOptions" :key="opt.value" :value="opt.value">
              {{ opt.label }}
            </option>
          </select>
        </div>
      </section>

      <!-- 数据管理 -->
      <section class="settings-section">
        <h2 class="section-title">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <ellipse cx="12" cy="5" rx="9" ry="3" />
            <path d="M3 5V19A9 3 0 0 0 21 19V5" />
            <path d="M3 12A9 3 0 0 0 21 12" />
          </svg>
          数据管理
        </h2>

        <div class="data-stats">
          <div class="stat-card">
            <span class="stat-number">{{ stats.diaries }}</span>
            <span class="stat-label">日记</span>
          </div>
          <div class="stat-card">
            <span class="stat-number">{{ stats.todos }}</span>
            <span class="stat-label">待办</span>
          </div>
          <div class="stat-card">
            <span class="stat-number">{{ stats.memos }}</span>
            <span class="stat-label">备忘</span>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">一键备份</span>
            <span class="setting-desc">将所有数据打包为 ZIP 文件下载</span>
          </div>
          <button class="action-btn primary" :disabled="isBackingUp" @click="handleBackup">
            {{ isBackingUp ? "备份中..." : "备份" }}
          </button>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">导出全部日记</span>
            <span class="setting-desc">将所有日记导出为 Markdown 文件</span>
          </div>
          <button class="action-btn" @click="handleExportAll">导出</button>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">恢复数据</span>
            <span class="setting-desc">从备份 ZIP 文件恢复数据</span>
          </div>
          <label class="action-btn">
            选择文件
            <input type="file" accept=".zip" @change="handleRestore" style="display: none" />
          </label>
        </div>

        <div v-if="backupStatus" class="backup-status">{{ backupStatus }}</div>
      </section>

      <!-- 外观 -->
      <section class="settings-section">
        <h2 class="section-title">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20M2 12h20" />
          </svg>
          外观
        </h2>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">主题模式</span>
            <span class="setting-desc">切换亮色/暗色主题</span>
          </div>
          <button class="action-btn" @click="appStore.toggleTheme()">
            {{ appStore.theme === "light" ? "切换到暗色" : "切换到亮色" }}
          </button>
        </div>
      </section>

      <!-- 关于 -->
      <section class="settings-section">
        <h2 class="section-title">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 16v-4M12 8h.01" />
          </svg>
          关于
        </h2>
        <div class="about-info">
          <h3 class="app-name">栖记 NestDiary</h3>
          <p class="app-version">版本 0.1.0 (MVP)</p>
          <p class="app-desc">
            隐私优先的本地日记 + 备忘 + 待办三合一桌面应用。<br />
            数据 100% 存储在本地，零账号、零云端、离线可用。
          </p>
          <div class="privacy-badges">
            <span class="badge">100% 本地存储</span>
            <span class="badge">零账号要求</span>
            <span class="badge">离线可用</span>
            <span class="badge">数据可移植</span>
          </div>
        </div>
      </section>
    </div>

    <!-- 密码设置弹窗 -->
    <Transition name="fade">
      <div v-if="showPasswordModal" class="modal-overlay" @click="showPasswordModal = false">
        <div class="modal" @click.stop>
          <h3 class="modal-title">
            {{ passwordMode === "set" ? "设置密码" : passwordMode === "change" ? "修改密码" : "关闭密码锁" }}
          </h3>

          <div v-if="passwordMode === 'change' || passwordMode === 'remove'" class="form-group">
            <label>原密码</label>
            <input v-model="oldPassword" type="password" class="form-input" autofocus />
          </div>

          <div v-if="passwordMode === 'set' || passwordMode === 'change'" class="form-group">
            <label>{{ passwordMode === "change" ? "新密码" : "密码" }}</label>
            <input v-model="newPassword" type="password" class="form-input" />
          </div>

          <div v-if="passwordMode === 'set' || passwordMode === 'change'" class="form-group">
            <label>确认密码</label>
            <input
              v-model="confirmPassword"
              type="password"
              class="form-input"
              @keyup.enter="handlePasswordSubmit"
            />
          </div>

          <p v-if="passwordError" class="form-error">{{ passwordError }}</p>

          <div class="modal-actions">
            <button class="modal-btn cancel" @click="showPasswordModal = false">取消</button>
            <button class="modal-btn confirm" @click="handlePasswordSubmit">确定</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.settings-view {
  height: 100%;
  overflow-y: auto;
  padding: 20px;
}

.settings-content {
  max-width: 700px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-section {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 20px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.setting-label {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.setting-desc {
  font-size: 12px;
  color: var(--text-muted);
}

.setting-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.action-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.action-btn.primary {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.action-btn.primary:hover {
  background: var(--primary-hover);
}

.action-btn.danger:hover {
  border-color: #ef4444;
  color: #ef4444;
  background: rgba(239, 68, 68, 0.05);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.select-input {
  height: 34px;
  padding: 0 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  cursor: pointer;
}

.data-stats {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.stat-card {
  flex: 1;
  background: var(--bg-color);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 16px;
  text-align: center;
}

.stat-number {
  display: block;
  font-size: 28px;
  font-weight: 700;
  color: var(--primary-color);
}

.stat-label {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 4px;
}

.backup-status {
  margin-top: 12px;
  padding: 8px 12px;
  background: rgba(99, 102, 241, 0.1);
  border-radius: var(--radius-sm);
  color: var(--primary-color);
  font-size: 13px;
}

.about-info {
  text-align: center;
  padding: 20px 0;
}

.app-name {
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.app-version {
  font-size: 13px;
  color: var(--text-muted);
  margin-bottom: 16px;
}

.app-desc {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.8;
  margin-bottom: 16px;
}

.privacy-badges {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 8px;
}

.badge {
  font-size: 11px;
  padding: 4px 12px;
  background: rgba(99, 102, 241, 0.1);
  color: var(--primary-color);
  border-radius: 12px;
  font-weight: 500;
}

/* 弹窗 */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
}

.modal {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  padding: 24px;
  width: 360px;
  box-shadow: var(--shadow-lg);
}

.modal-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.form-input {
  width: 100%;
  height: 38px;
  padding: 0 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.form-input:focus {
  border-color: var(--primary-color);
}

.form-error {
  color: #ef4444;
  font-size: 13px;
  margin-bottom: 12px;
}

.modal-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.modal-btn {
  padding: 8px 20px;
  border: none;
  border-radius: var(--radius-sm);
  font-size: 13px;
  cursor: pointer;
}

.modal-btn.cancel {
  background: var(--bg-sidebar);
  color: var(--text-secondary);
}

.modal-btn.confirm {
  background: var(--primary-color);
  color: white;
}

.modal-btn.confirm:hover {
  background: var(--primary-hover);
}
</style>
