<script setup lang="ts">
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
import { searchAll } from "../db/database";
import type { SearchResult } from "../types";

const router = useRouter();
const keyword = ref("");
const results = ref<SearchResult[]>([]);
const isSearching = ref(false);
const filterType = ref<"all" | "diary" | "todo" | "memo">("all");

let searchTimer: number | null = null;

async function doSearch() {
  if (!keyword.value.trim()) {
    results.value = [];
    return;
  }
  isSearching.value = true;
  results.value = await searchAll(keyword.value);
  isSearching.value = false;
}

watch(keyword, () => {
  if (searchTimer) clearTimeout(searchTimer);
  searchTimer = window.setTimeout(doSearch, 300);
});

const filteredResults = ref<SearchResult[]>([]);
watch([results, filterType], () => {
  if (filterType.value === "all") {
    filteredResults.value = results.value;
  } else {
    filteredResults.value = results.value.filter((r) => r.type === filterType.value);
  }
}, { immediate: true });

function handleResultClick(result: SearchResult) {
  if (result.type === "diary") {
    // 跳转到日记（需要通过 URL 参数传递日期）
    router.push({ path: "/diary", query: { date: result.date } });
  } else if (result.type === "todo") {
    router.push("/todo");
  } else if (result.type === "memo") {
    router.push("/memo");
  }
}

function highlightKeyword(text: string, kw: string): string {
  if (!kw) return text;
  const regex = new RegExp(`(${kw.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")})`, "gi");
  return text.replace(regex, '<mark class="highlight">$1</mark>');
}

const typeLabels: Record<string, string> = {
  diary: "日记",
  todo: "待办",
  memo: "备忘",
};
</script>

<template>
  <div class="search-view">
    <!-- 搜索框 -->
    <div class="search-header">
      <div class="big-search-box">
        <svg class="search-icon" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <input
          v-model="keyword"
          type="text"
          placeholder="输入关键词搜索日记、备忘、待办..."
          class="search-input"
          autofocus
        />
        <span v-if="isSearching" class="searching-indicator">搜索中...</span>
      </div>
    </div>

    <!-- 筛选 -->
    <div class="filter-bar" v-if="results.length > 0">
      <button
        class="filter-btn"
        :class="{ active: filterType === 'all' }"
        @click="filterType = 'all'"
      >
        全部 <span class="count">{{ results.length }}</span>
      </button>
      <button
        class="filter-btn"
        :class="{ active: filterType === 'diary' }"
        @click="filterType = 'diary'"
      >
        日记 <span class="count">{{ results.filter(r => r.type === 'diary').length }}</span>
      </button>
      <button
        class="filter-btn"
        :class="{ active: filterType === 'todo' }"
        @click="filterType = 'todo'"
      >
        待办 <span class="count">{{ results.filter(r => r.type === 'todo').length }}</span>
      </button>
      <button
        class="filter-btn"
        :class="{ active: filterType === 'memo' }"
        @click="filterType = 'memo'"
      >
        备忘 <span class="count">{{ results.filter(r => r.type === 'memo').length }}</span>
      </button>
    </div>

    <!-- 搜索结果 -->
    <div class="search-results-container">
      <div v-if="keyword && !isSearching && filteredResults.length === 0" class="no-results">
        <p>未找到与 "{{ keyword }}" 相关的内容</p>
      </div>

      <div v-if="!keyword && !isSearching" class="search-empty">
        <svg viewBox="0 0 24 24" width="64" height="64" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <p>输入关键词开始搜索</p>
        <p class="empty-hint">支持搜索日记正文、标题、待办内容、备忘内容</p>
      </div>

      <TransitionGroup name="list" tag="div" class="result-list">
        <div
          v-for="result in filteredResults"
          :key="`${result.type}-${result.id}`"
          class="result-item"
          @click="handleResultClick(result)"
        >
          <span class="result-type-badge" :class="result.type">
            {{ typeLabels[result.type] }}
          </span>
          <div class="result-body">
            <div class="result-title" v-html="highlightKeyword(result.title, keyword)"></div>
            <div class="result-snippet" v-html="highlightKeyword(result.snippet, keyword)"></div>
          </div>
          <span class="result-date">{{ result.date }}</span>
        </div>
      </TransitionGroup>
    </div>
  </div>
</template>

<style scoped>
.search-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.search-header {
  padding: 20px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.big-search-box {
  position: relative;
  max-width: 700px;
  margin: 0 auto;
}

.search-icon {
  position: absolute;
  left: 14px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
}

.search-input {
  width: 100%;
  height: 44px;
  padding: 0 16px 0 44px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 15px;
  outline: none;
  transition: border-color 0.2s;
}

.search-input:focus {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.searching-indicator {
  position: absolute;
  right: 14px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 12px;
  color: var(--text-muted);
}

.filter-bar {
  display: flex;
  gap: 8px;
  padding: 12px 20px;
  max-width: 700px;
  margin: 0 auto;
  width: 100%;
}

.filter-btn {
  padding: 4px 12px;
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  border-radius: 16px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
  display: flex;
  align-items: center;
  gap: 4px;
}

.filter-btn:hover {
  border-color: var(--primary-color);
}

.filter-btn.active {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.filter-btn .count {
  font-size: 11px;
  opacity: 0.7;
}

.search-results-container {
  flex: 1;
  overflow-y: auto;
  padding: 0 20px 20px;
  max-width: 700px;
  margin: 0 auto;
  width: 100%;
}

.result-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.result-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 14px 16px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.15s;
}

.result-item:hover {
  border-color: var(--primary-color);
  box-shadow: var(--shadow-sm);
}

.result-type-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
  flex-shrink: 0;
  margin-top: 2px;
}

.result-type-badge.diary {
  background: rgba(99, 102, 241, 0.15);
  color: var(--primary-color);
}

.result-type-badge.todo {
  background: rgba(34, 197, 94, 0.15);
  color: #22c55e;
}

.result-type-badge.memo {
  background: rgba(245, 158, 11, 0.15);
  color: #f59e0b;
}

.result-body {
  flex: 1;
  min-width: 0;
}

.result-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-snippet {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.result-date {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
  white-space: nowrap;
}

:deep(.highlight) {
  background: rgba(245, 158, 11, 0.25);
  color: #d97706;
  border-radius: 2px;
  padding: 0 2px;
}

.search-empty,
.no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted);
  text-align: center;
}

.search-empty p,
.no-results p {
  margin-top: 12px;
  font-size: 14px;
}

.empty-hint {
  font-size: 12px !important;
  color: var(--text-muted);
}

/* 列表动画 */
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.list-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
