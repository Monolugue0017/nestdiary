<script setup lang="ts">
import { ref } from "vue";
import { useAppStore } from "../stores/app";

const appStore = useAppStore();
const password = ref("");
const errorMsg = ref("");
const errorCount = ref(0);
const isLocked = ref(false); // 错误3次后临时锁定
const lockCountdown = ref(0);

async function handleUnlock() {
  if (isLocked.value) return;

  const success = await appStore.unlock(password.value);
  if (success) {
    password.value = "";
    errorMsg.value = "";
    errorCount.value = 0;
  } else {
    errorCount.value++;
    if (errorCount.value >= 3) {
      isLocked.value = true;
      lockCountdown.value = 30;
      errorMsg.value = "密码错误次数过多，请等待 30 秒后重试";
      const timer = setInterval(() => {
        lockCountdown.value--;
        if (lockCountdown.value <= 0) {
          clearInterval(timer);
          isLocked.value = false;
          errorCount.value = 0;
          errorMsg.value = "";
        }
      }, 1000);
    } else {
      errorMsg.value = `密码错误，还剩 ${3 - errorCount.value} 次尝试机会`;
    }
    password.value = "";
  }
}
</script>

<template>
  <div class="lock-screen">
    <div class="lock-container">
      <!-- 锁图标 -->
      <div class="lock-icon">
        <svg viewBox="0 0 24 24" width="48" height="48" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
          <path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
      </div>

      <h2 class="lock-title">栖记</h2>
      <p class="lock-subtitle">请输入密码解锁应用</p>

      <div class="input-group">
        <input
          v-model="password"
          type="password"
          :placeholder="isLocked ? `请等待 ${lockCountdown} 秒` : '输入密码'"
          :disabled="isLocked"
          class="password-input"
          @keyup.enter="handleUnlock"
          autofocus
        />
        <button class="unlock-btn" :disabled="isLocked || !password" @click="handleUnlock">
          解锁
        </button>
      </div>

      <Transition name="fade">
        <p v-if="errorMsg" class="error-msg">{{ errorMsg }}</p>
      </Transition>

      <p class="lock-hint">数据 100% 存储在本地，零云端，零账号</p>
    </div>
  </div>
</template>

<style scoped>
.lock-screen {
  position: fixed;
  inset: 0;
  background: var(--bg-color);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.lock-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 360px;
}

.lock-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--bg-card);
  border: 2px solid var(--primary-color);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--primary-color);
  margin-bottom: 24px;
  box-shadow: var(--shadow-md);
}

.lock-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.lock-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 32px;
}

.input-group {
  display: flex;
  gap: 8px;
  width: 100%;
  margin-bottom: 16px;
}

.password-input {
  flex: 1;
  height: 42px;
  padding: 0 14px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-card);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.password-input:focus {
  border-color: var(--primary-color);
}

.password-input:disabled {
  opacity: 0.5;
}

.unlock-btn {
  height: 42px;
  padding: 0 20px;
  border: none;
  border-radius: var(--radius-md);
  background: var(--primary-color);
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.unlock-btn:hover:not(:disabled) {
  background: var(--primary-hover);
}

.unlock-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-msg {
  color: #ef4444;
  font-size: 13px;
  margin-bottom: 16px;
}

.lock-hint {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 32px;
}
</style>
