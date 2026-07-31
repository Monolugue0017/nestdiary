<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getMemos, addMemo, updateMemo, deleteMemo } from "../db/database";
import { formatTimestamp } from "../utils/date";
import type { Memo } from "../types";

const memos = ref<Memo[]>([]);
const newMemoContent = ref("");
const editingId = ref<number | null>(null);
const editingContent = ref("");

async function loadMemos() {
  memos.value = await getMemos();
}

async function handleAddMemo() {
  const content = newMemoContent.value.trim();
  if (!content) return;
  await addMemo(content);
  newMemoContent.value = "";
  await loadMemos();
}

async function togglePin(memo: Memo) {
  await updateMemo(memo.id!, { pinned: !memo.pinned });
  await loadMemos();
}

async function handleDelete(id: number) {
  await deleteMemo(id);
  await loadMemos();
}

function startEdit(memo: Memo) {
  editingId.value = memo.id!;
  editingContent.value = memo.content;
}

async function saveEdit() {
  if (editingId.value !== null && editingContent.value.trim()) {
    await updateMemo(editingId.value, { content: editingContent.value.trim() });
  }
  editingId.value = null;
  editingContent.value = "";
  await loadMemos();
}

function cancelEdit() {
  editingId.value = null;
  editingContent.value = "";
}

onMounted(loadMemos);
</script>

<template>
  <div class="memo-view">
    <!-- 添加备忘 -->
    <div class="add-section">
      <textarea
        v-model="newMemoContent"
        class="memo-input"
        placeholder="快速记录一条备忘..."
        rows="2"
        @keydown.ctrl.enter="handleAddMemo"
      ></textarea>
      <div class="add-bar">
        <span class="hint">Ctrl + Enter 快速保存</span>
        <button class="add-btn" @click="handleAddMemo" :disabled="!newMemoContent.trim()">
          添加备忘
        </button>
      </div>
    </div>

    <!-- 备忘列表 -->
    <div class="memo-list-container">
      <div class="memo-grid" v-if="memos.length > 0">
        <TransitionGroup name="card">
          <div
            v-for="memo in memos"
            :key="memo.id"
            class="memo-card"
            :class="{ pinned: memo.pinned }"
          >
            <!-- 置顶图标 -->
            <button
              class="pin-btn"
              :class="{ active: memo.pinned }"
              @click="togglePin(memo)"
              title="置顶/取消置顶"
            >
              <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 17v5M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z" />
              </svg>
            </button>

            <!-- 内容 -->
            <div v-if="editingId === memo.id" class="memo-edit">
              <textarea
                v-model="editingContent"
                class="edit-textarea"
                rows="3"
                autofocus
              ></textarea>
              <div class="edit-actions">
                <button class="edit-btn save" @click="saveEdit">保存</button>
                <button class="edit-btn cancel" @click="cancelEdit">取消</button>
              </div>
            </div>
            <div v-else class="memo-content" @dblclick="startEdit(memo)">
              {{ memo.content }}
            </div>

            <!-- 底部 -->
            <div class="memo-footer">
              <span class="memo-time">{{ formatTimestamp(memo.updatedAt) }}</span>
              <div class="memo-actions">
                <button class="action-btn" @click="startEdit(memo)" title="编辑">
                  <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                    <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
                  </svg>
                </button>
                <button class="action-btn delete" @click="handleDelete(memo.id!)" title="删除">
                  <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </TransitionGroup>
      </div>

      <!-- 空状态 -->
      <div v-else class="empty-state">
        <svg viewBox="0 0 24 24" width="64" height="64" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <path d="M14 2v6h6M16 13H8M16 17H8M10 9H8" />
        </svg>
        <p>暂无备忘录</p>
        <p class="empty-hint">在上方输入框中记录你的第一条备忘</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.memo-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.add-section {
  padding: 16px 20px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
  max-width: 900px;
  margin: 0 auto;
  width: 100%;
}

.memo-input {
  width: 100%;
  padding: 10px 14px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  resize: none;
  font-family: inherit;
  transition: border-color 0.2s;
}

.memo-input:focus {
  border-color: var(--primary-color);
}

.add-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 8px;
}

.hint {
  font-size: 12px;
  color: var(--text-muted);
}

.add-btn {
  height: 32px;
  padding: 0 16px;
  border: none;
  border-radius: var(--radius-md);
  background: var(--primary-color);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.add-btn:hover:not(:disabled) {
  background: var(--primary-hover);
}

.add-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.memo-list-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
  width: 100%;
}

.memo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 12px;
}

.memo-card {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 14px;
  position: relative;
  transition: all 0.2s;
}

.memo-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--primary-color);
}

.memo-card.pinned {
  border-color: var(--primary-color);
  background: rgba(99, 102, 241, 0.03);
}

.pin-btn {
  position: absolute;
  top: 10px;
  right: 10px;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.pin-btn:hover {
  color: var(--primary-color);
  background: var(--bg-sidebar);
}

.pin-btn.active {
  color: var(--primary-color);
}

.memo-content {
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
  min-height: 40px;
  padding-right: 24px;
  cursor: pointer;
}

.memo-edit {
  padding-right: 24px;
}

.edit-textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid var(--primary-color);
  border-radius: var(--radius-sm);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  resize: vertical;
  font-family: inherit;
}

.edit-actions {
  display: flex;
  gap: 6px;
  margin-top: 6px;
}

.edit-btn {
  padding: 4px 12px;
  border: none;
  border-radius: var(--radius-sm);
  font-size: 12px;
  cursor: pointer;
}

.edit-btn.save {
  background: var(--primary-color);
  color: white;
}

.edit-btn.cancel {
  background: var(--bg-sidebar);
  color: var(--text-secondary);
}

.memo-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 10px;
  padding-top: 8px;
  border-top: 1px solid var(--border-color);
}

.memo-time {
  font-size: 11px;
  color: var(--text-muted);
}

.memo-actions {
  display: flex;
  gap: 4px;
}

.action-btn {
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: all 0.15s;
}

.memo-card:hover .action-btn {
  opacity: 1;
}

.action-btn:hover {
  background: var(--bg-sidebar);
  color: var(--text-primary);
}

.action-btn.delete:hover {
  color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted);
}

.empty-state p {
  margin-top: 12px;
  font-size: 14px;
}

.empty-hint {
  font-size: 12px !important;
}

/* 卡片动画 */
.card-enter-active,
.card-leave-active {
  transition: all 0.3s ease;
}

.card-enter-from {
  opacity: 0;
  transform: scale(0.9);
}

.card-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
</style>
