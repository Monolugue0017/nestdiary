<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import DiaryEditor from "../components/DiaryEditor.vue";
import { getDiaryByDate, saveDiary } from "../db/database";
import { today, formatDate, formatDateChinese, addDays, isToday, parseDate } from "../utils/date";
import { exportDiaryAsMarkdown } from "../utils/export";
import type { Diary } from "../types";

const route = useRoute();

// 当前日期
const currentDate = ref(today());
const dateDisplay = ref("");

// 日记内容
const diaryTitle = ref("");
const diaryContent = ref("");
const diaryPlainText = ref("");
const diaryWordCount = ref(0);

// 状态
const isSaved = ref(true);
const saveStatus = ref("已保存");
const currentDiaryId = ref<number | undefined>(undefined);

// 自动保存计时器
let saveTimer: number | null = null;

// 加载日记
async function loadDiary(date: string) {
  // 如果有未保存的内容，先保存
  if (saveTimer) {
    clearTimeout(saveTimer);
    await doSave();
  }

  currentDate.value = date;
  dateDisplay.value = formatDateChinese(parseDate(date));

  const existing = await getDiaryByDate(date);
  if (existing) {
    currentDiaryId.value = existing.id;
    diaryTitle.value = existing.title || "";
    diaryContent.value = existing.content || "";
    diaryPlainText.value = existing.plainText || "";
    diaryWordCount.value = existing.wordCount || 0;
  } else {
    currentDiaryId.value = undefined;
    diaryTitle.value = "";
    diaryContent.value = "";
    diaryPlainText.value = "";
    diaryWordCount.value = 0;
  }
  isSaved.value = true;
  saveStatus.value = "已保存";
}

// 内容变化时触发自动保存
function onContentChange() {
  isSaved.value = false;
  saveStatus.value = "编辑中...";

  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = window.setTimeout(() => {
    doSave();
  }, 2000); // 停止输入2秒后自动保存
}

// 执行保存
async function doSave() {
  // 如果没有任何内容且没有已存在的日记，不保存
  if (!diaryTitle.value && !diaryContent.value && !currentDiaryId.value) {
    isSaved.value = true;
    saveStatus.value = "已保存";
    return;
  }

  saveStatus.value = "保存中...";

  const diary: Diary = {
    id: currentDiaryId.value,
    date: currentDate.value,
    title: diaryTitle.value,
    content: diaryContent.value,
    plainText: diaryPlainText.value,
    wordCount: diaryWordCount.value,
    createdAt: Date.now(),
    updatedAt: Date.now(),
  };

  const id = await saveDiary(diary);
  currentDiaryId.value = id;
  isSaved.value = true;
  saveStatus.value = "已保存";
}

// 日期导航
function prevDay() {
  loadDiary(addDays(currentDate.value, -1));
}
function nextDay() {
  loadDiary(addDays(currentDate.value, 1));
}
function goToToday() {
  loadDiary(today());
}

// 导出当前日记
function exportCurrent() {
  if (!diaryContent.value && !diaryTitle.value) return;
  exportDiaryAsMarkdown(diaryTitle.value, currentDate.value, diaryContent.value);
}

// 监听内容变化
watch(diaryContent, onContentChange);
watch(diaryTitle, onContentChange);

// 日期选择器
const showDatePicker = ref(false);
const pickerDate = ref(today());

function applyPickerDate() {
  loadDiary(pickerDate.value);
  showDatePicker.value = false;
}

// 组件卸载时保存
onUnmounted(() => {
  if (saveTimer) {
    clearTimeout(saveTimer);
    doSave();
  }
});

// 初始化
onMounted(() => {
  const queryDate = route.query.date as string;
  loadDiary(queryDate || today());
});

// 监听路由日期参数变化
watch(() => route.query.date, (newDate) => {
  if (newDate && typeof newDate === "string") {
    loadDiary(newDate);
  }
});
</script>

<template>
  <div class="diary-view">
    <!-- 日期导航栏 -->
    <div class="date-bar">
      <button class="nav-btn" title="前一天" @click="prevDay">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 18 9 12 15 6" />
        </svg>
      </button>

      <div class="date-display" @click="showDatePicker = !showDatePicker">
        <span class="date-text">{{ dateDisplay }}</span>
        <span v-if="isToday(currentDate)" class="today-badge">今天</span>
        <svg class="calendar-icon" viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
          <line x1="16" y1="2" x2="16" y2="6" />
          <line x1="8" y1="2" x2="8" y2="6" />
          <line x1="3" y1="10" x2="21" y2="10" />
        </svg>
      </div>

      <button class="nav-btn" title="后一天" @click="nextDay">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="9 18 15 12 9 6" />
        </svg>
      </button>

      <button class="today-btn" @click="goToToday">回到今天</button>

      <div class="date-bar-right">
        <span class="word-count">{{ diaryWordCount }} 字</span>
        <span class="save-status" :class="{ saved: isSaved, saving: !isSaved }">
          <span class="status-dot"></span>
          {{ saveStatus }}
        </span>
        <button class="export-btn" title="导出为 Markdown" @click="exportCurrent">
          <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
          </svg>
        </button>
      </div>
    </div>

    <!-- 日期选择器弹窗 -->
    <Transition name="fade">
      <div v-if="showDatePicker" class="date-picker-overlay" @click="showDatePicker = false">
        <div class="date-picker" @click.stop>
          <input type="date" v-model="pickerDate" class="date-input" />
          <button class="picker-confirm" @click="applyPickerDate">跳转</button>
        </div>
      </div>
    </Transition>

    <!-- 编辑区域 -->
    <div class="diary-content">
      <input
        v-model="diaryTitle"
        class="diary-title-input"
        type="text"
        placeholder="给今天起个标题（可选）..."
      />
      <DiaryEditor
        v-model="diaryContent"
        @update:plain-text="diaryPlainText = $event"
        @update:word-count="diaryWordCount = $event"
      />
    </div>
  </div>
</template>

<style scoped>
.diary-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.date-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.nav-btn {
  width: 32px;
  height: 32px;
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.nav-btn:hover {
  background: var(--bg-sidebar);
  color: var(--primary-color);
  border-color: var(--primary-color);
}

.date-display {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 12px;
  border-radius: var(--radius-sm);
  transition: background 0.15s;
}

.date-display:hover {
  background: var(--bg-sidebar);
}

.date-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.today-badge {
  font-size: 11px;
  padding: 2px 8px;
  background: var(--primary-color);
  color: white;
  border-radius: 10px;
  font-weight: 500;
}

.calendar-icon {
  color: var(--text-muted);
}

.today-btn {
  padding: 4px 12px;
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

.today-btn:hover {
  background: var(--primary-color);
  color: white;
  border-color: var(--primary-color);
}

.date-bar-right {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: 12px;
}

.word-count {
  font-size: 12px;
  color: var(--text-muted);
}

.save-status {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.save-status.saved .status-dot {
  background: #22c55e;
}

.save-status.saving .status-dot {
  background: #f59e0b;
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.export-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.export-btn:hover {
  background: var(--bg-sidebar);
  color: var(--primary-color);
}

.date-picker-overlay {
  position: fixed;
  inset: 0;
  z-index: 200;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 100px;
  background: rgba(0, 0, 0, 0.1);
}

.date-picker {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 16px;
  display: flex;
  gap: 8px;
  box-shadow: var(--shadow-lg);
}

.date-input {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.picker-confirm {
  padding: 6px 16px;
  border: none;
  background: var(--primary-color);
  color: white;
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
}

.picker-confirm:hover {
  background: var(--primary-hover);
}

.diary-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
  width: 100%;
}

.diary-title-input {
  width: 100%;
  border: none;
  outline: none;
  background: transparent;
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary);
  padding: 8px 0 16px;
  margin-bottom: 8px;
  border-bottom: 1px solid var(--border-color);
}

.diary-title-input::placeholder {
  color: var(--text-muted);
  font-weight: 400;
}
</style>
