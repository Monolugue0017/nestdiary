<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../stores/app";

const router = useRouter();
const appStore = useAppStore();

const currentRoute = ref(router.currentRoute);

const navItems = [
  { path: "/diary", icon: "book", label: "日记", shortcut: "Ctrl+1" },
  { path: "/todo", icon: "check", label: "待办", shortcut: "Ctrl+2" },
  { path: "/memo", icon: "note", label: "备忘", shortcut: "Ctrl+3" },
  { path: "/search", icon: "search", label: "搜索", shortcut: "Ctrl+K" },
  { path: "/settings", icon: "settings", label: "设置", shortcut: "" },
];

function navigate(path: string) {
  router.push(path);
  appStore.updateActivity();
}
</script>

<template>
  <nav class="sidebar">
    <!-- Logo -->
    <div class="logo" @click="navigate('/diary')">
      <svg viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M4 4h16v3H4z" />
        <path d="M6 7v13h12V7" />
        <path d="M9 11h6M9 15h4" />
      </svg>
    </div>

    <!-- 导航项 -->
    <div class="nav-items">
      <button
        v-for="item in navItems"
        :key="item.path"
        class="nav-item"
        :class="{ active: currentRoute.path === item.path }"
        :title="item.label + (item.shortcut ? ' (' + item.shortcut + ')' : '')"
        @click="navigate(item.path)"
      >
        <!-- 日记图标 -->
        <svg v-if="item.icon === 'book'" viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
          <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />
        </svg>
        <!-- 待办图标 -->
        <svg v-else-if="item.icon === 'check'" viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M9 11l3 3L22 4" />
          <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
        </svg>
        <!-- 备忘图标 -->
        <svg v-else-if="item.icon === 'note'" viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <path d="M14 2v6h6M16 13H8M16 17H8M10 9H8" />
        </svg>
        <!-- 搜索图标 -->
        <svg v-else-if="item.icon === 'search'" viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <!-- 设置图标 -->
        <svg v-else-if="item.icon === 'settings'" viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
          <circle cx="12" cy="12" r="3" />
        </svg>
      </button>
    </div>

    <!-- 底部：锁定按钮 -->
    <div class="sidebar-footer">
      <button
        v-if="appStore.hasPassword"
        class="nav-item lock-btn"
        title="锁定应用"
        @click="appStore.lock()"
      >
        <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
          <path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
      </button>
    </div>
  </nav>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100%;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
  flex-shrink: 0;
}

.logo {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--primary-color);
  cursor: pointer;
  margin-bottom: 16px;
  transition: transform 0.2s;
}

.logo:hover {
  transform: scale(1.1);
}

.nav-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
}

.nav-item {
  width: 40px;
  height: 40px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  position: relative;
}

.nav-item:hover {
  background: rgba(99, 102, 241, 0.1);
  color: var(--primary-color);
}

.nav-item.active {
  background: var(--primary-color);
  color: white;
}

.nav-item.active::before {
  content: "";
  position: absolute;
  left: -12px;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background: var(--primary-color);
  border-radius: 2px;
}

.sidebar-footer {
  margin-top: auto;
}

.lock-btn:hover {
  color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}
</style>
