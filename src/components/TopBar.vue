<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { searchAll } from "../db/database";
import type { SearchResult } from "../types";
import { useAppStore } from "../stores/app";

const router = useRouter();
const appStore = useAppStore();

const searchKeyword = ref("");
const searchResults = ref<SearchResult[]>([]);
const showResults = ref(false);
const isSearching = ref(false);

let searchTimer: number | null = null;

function onSearchInput() {
  if (searchTimer) clearTimeout(searchTimer);
  if (!searchKeyword.value.trim()) {
    searchResults.value = [];
    showResults.value = false;
    return;
  }
  searchTimer = window.setTimeout(async () => {
    isSearching.value = true;
    searchResults.value = await searchAll(searchKeyword.value);
    showResults.value = true;
    isSearching.value = false;
  }, 300); // 防抖 300ms
}

function goToSearch() {
  router.push("/search");
}

function onBlurDelay() {
  window.setTimeout(() => {
    showResults.value = false;
  }, 200);
}

const currentTitle = computed(() => {
  const path = router.currentRoute.value.path;
  const map: Record<string, string> = {
    "/diary": "日记",
    "/todo": "待办",
    "/memo": "备忘",
    "/search": "搜索",
    "/settings": "设置",
  };
  return map[path] || "";
});
</script>

<template>
  <header class="topbar">
    <div class="topbar-left">
      <h1 class="page-title">{{ currentTitle }}</h1>
    </div>

    <div class="topbar-center">
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <input
          v-model="searchKeyword"
          type="text"
          placeholder="搜索日记、备忘、待办... (Ctrl+K)"
          @input="onSearchInput"
          @focus="showResults = searchResults.length > 0"
          @blur="onBlurDelay"
        />
        <!-- 搜索结果下拉 -->
        <div v-if="showResults && searchResults.length > 0" class="search-results">
          <div
            v-for="result in searchResults.slice(0, 8)"
            :key="`${result.type}-${result.id}`"
            class="search-result-item"
            @mousedown="goToSearch()"
          >
            <span class="result-type" :class="result.type">{{ result.type === 'diary' ? '日记' : result.type === 'todo' ? '待办' : '备忘' }}</span>
            <span class="result-title">{{ result.title }}</span>
            <span class="result-date">{{ result.date }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="topbar-right">
      <button class="icon-btn" title="切换主题" @click="appStore.toggleTheme()">
        <svg v-if="appStore.theme === 'light'" viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
        </svg>
        <svg v-else viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="4" />
          <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" />
        </svg>
      </button>
    </div>
  </header>
</template>

<style scoped>
.topbar {
  height: var(--topbar-height);
  background: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  padding: 0 16px;
  gap: 16px;
  flex-shrink: 0;
}

.topbar-left {
  flex-shrink: 0;
}

.page-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.topbar-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.search-box {
  position: relative;
  width: 100%;
  max-width: 500px;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
}

.search-box input {
  width: 100%;
  height: 36px;
  padding: 0 12px 0 36px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.2s;
}

.search-box input:focus {
  border-color: var(--primary-color);
}

.search-results {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  max-height: 400px;
  overflow-y: auto;
  z-index: 100;
}

.search-result-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  cursor: pointer;
  transition: background 0.15s;
  border-bottom: 1px solid var(--border-color);
}

.search-result-item:last-child {
  border-bottom: none;
}

.search-result-item:hover {
  background: var(--bg-sidebar);
}

.result-type {
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 500;
  flex-shrink: 0;
}

.result-type.diary {
  background: rgba(99, 102, 241, 0.15);
  color: var(--primary-color);
}

.result-type.todo {
  background: rgba(34, 197, 94, 0.15);
  color: #22c55e;
}

.result-type.memo {
  background: rgba(245, 158, 11, 0.15);
  color: #f59e0b;
}

.result-title {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-date {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.topbar-right {
  flex-shrink: 0;
}

.icon-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: var(--bg-sidebar);
  color: var(--text-primary);
}
</style>
