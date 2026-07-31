<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getTodos, addTodo, updateTodo, deleteTodo } from "../db/database";
import { formatTimestamp } from "../utils/date";
import type { Todo } from "../types";

const todos = ref<Todo[]>([]);
const newTodoText = ref("");
const newTodoPriority = ref<1 | 2 | 3>(2);
const showCompleted = ref(true);

const priorityLabels: Record<number, { label: string; color: string }> = {
  1: { label: "高", color: "#ef4444" },
  2: { label: "中", color: "#f59e0b" },
  3: { label: "低", color: "#6b7280" },
};

async function loadTodos() {
  todos.value = await getTodos();
}

async function handleAddTodo() {
  const text = newTodoText.value.trim();
  if (!text) return;
  await addTodo(text, newTodoPriority.value);
  newTodoText.value = "";
  await loadTodos();
}

async function toggleComplete(todo: Todo) {
  await updateTodo(todo.id!, {
    completed: !todo.completed,
    completedAt: !todo.completed ? Date.now() : undefined,
  });
  await loadTodos();
}

async function changePriority(todo: Todo, priority: 1 | 2 | 3) {
  await updateTodo(todo.id!, { priority });
  await loadTodos();
}

async function handleDelete(todo: Todo) {
  await deleteTodo(todo.id!);
  await loadTodos();
}

const activeTodos = ref<Todo[]>([]);
const completedTodos = ref<Todo[]>([]);

function updateFiltered() {
  activeTodos.value = todos.value.filter((t) => !t.completed);
  completedTodos.value = todos.value.filter((t) => t.completed);
}

import { watch } from "vue";
watch(todos, updateFiltered, { deep: true });

onMounted(loadTodos);
</script>

<template>
  <div class="todo-view">
    <!-- 添加待办 -->
    <div class="add-section">
      <div class="add-row">
        <select v-model="newTodoPriority" class="priority-select">
          <option :value="1">高优先级</option>
          <option :value="2">中优先级</option>
          <option :value="3">低优先级</option>
        </select>
        <input
          v-model="newTodoText"
          type="text"
          class="todo-input"
          placeholder="输入待办事项，按回车添加..."
          @keyup.enter="handleAddTodo"
        />
        <button class="add-btn" @click="handleAddTodo" :disabled="!newTodoText.trim()">
          添加
        </button>
      </div>
    </div>

    <!-- 待办列表 -->
    <div class="todo-list-container">
      <!-- 进行中 -->
      <div class="todo-section" v-if="activeTodos.length > 0">
        <div class="section-header">
          <span class="section-title">进行中</span>
          <span class="section-count">{{ activeTodos.length }}</span>
        </div>
        <TransitionGroup name="list" tag="div" class="todo-items">
          <div v-for="todo in activeTodos" :key="todo.id" class="todo-item">
            <button
              class="check-btn"
              :class="{ checked: todo.completed }"
              @click="toggleComplete(todo)"
            >
              <svg v-if="todo.completed" viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12" />
              </svg>
            </button>
            <div class="todo-main">
              <span class="todo-text">{{ todo.text }}</span>
              <div class="todo-meta">
                <span
                  class="priority-tag"
                  :style="{ color: priorityLabels[todo.priority].color, borderColor: priorityLabels[todo.priority].color }"
                  @click="changePriority(todo, todo.priority === 1 ? 2 : todo.priority === 2 ? 3 : 1)"
                >
                  {{ priorityLabels[todo.priority].label }}
                </span>
                <span class="todo-time">{{ formatTimestamp(todo.createdAt) }}</span>
              </div>
            </div>
            <button class="delete-btn" @click="handleDelete(todo)">
              <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              </svg>
            </button>
          </div>
        </TransitionGroup>
      </div>

      <!-- 已完成 -->
      <div class="todo-section" v-if="completedTodos.length > 0 && showCompleted">
        <div class="section-header">
          <span class="section-title completed-title">已完成</span>
          <span class="section-count">{{ completedTodos.length }}</span>
          <button class="toggle-completed" @click="showCompleted = false">隐藏</button>
        </div>
        <TransitionGroup name="list" tag="div" class="todo-items">
          <div v-for="todo in completedTodos" :key="todo.id" class="todo-item completed">
            <button
              class="check-btn checked"
              @click="toggleComplete(todo)"
            >
              <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12" />
              </svg>
            </button>
            <div class="todo-main">
              <span class="todo-text completed-text">{{ todo.text }}</span>
              <div class="todo-meta">
                <span class="todo-time">{{ formatTimestamp(todo.completedAt || todo.updatedAt) }}</span>
              </div>
            </div>
            <button class="delete-btn" @click="handleDelete(todo)">
              <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              </svg>
            </button>
          </div>
        </TransitionGroup>
      </div>

      <!-- 隐藏已完成时显示展开按钮 -->
      <div v-if="completedTodos.length > 0 && !showCompleted" class="show-completed-bar">
        <button @click="showCompleted = true">显示已完成 ({{ completedTodos.length }})</button>
      </div>

      <!-- 空状态 -->
      <div v-if="todos.length === 0" class="empty-state">
        <svg viewBox="0 0 24 24" width="64" height="64" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M9 11l3 3L22 4" />
          <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
        </svg>
        <p>暂无待办事项</p>
        <p class="empty-hint">在上方输入框中添加你的第一个待办</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.todo-view {
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
}

.add-row {
  display: flex;
  gap: 8px;
  max-width: 800px;
  margin: 0 auto;
}

.priority-select {
  height: 38px;
  padding: 0 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  cursor: pointer;
}

.todo-input {
  flex: 1;
  height: 38px;
  padding: 0 14px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-color);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.todo-input:focus {
  border-color: var(--primary-color);
}

.add-btn {
  height: 38px;
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

.add-btn:hover:not(:disabled) {
  background: var(--primary-hover);
}

.add-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.todo-list-container {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
  max-width: 800px;
  margin: 0 auto;
  width: 100%;
}

.todo-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color);
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.completed-title {
  color: var(--text-muted);
}

.section-count {
  font-size: 12px;
  color: var(--text-muted);
  background: var(--bg-sidebar);
  padding: 2px 8px;
  border-radius: 10px;
}

.toggle-completed {
  margin-left: auto;
  border: none;
  background: transparent;
  color: var(--text-muted);
  font-size: 12px;
  cursor: pointer;
}

.toggle-completed:hover {
  color: var(--primary-color);
}

.todo-items {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.todo-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  transition: all 0.15s;
}

.todo-item:hover {
  border-color: var(--primary-color);
  box-shadow: var(--shadow-sm);
}

.todo-item.completed {
  opacity: 0.6;
}

.check-btn {
  width: 20px;
  height: 20px;
  border: 2px solid var(--border-color);
  border-radius: 50%;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  margin-top: 2px;
  transition: all 0.2s;
}

.check-btn.checked {
  background: #22c55e;
  border-color: #22c55e;
}

.todo-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.todo-text {
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.5;
}

.completed-text {
  text-decoration: line-through;
  color: var(--text-muted);
}

.todo-meta {
  display: flex;
  align-items: center;
  gap: 8px;
}

.priority-tag {
  font-size: 11px;
  padding: 1px 8px;
  border: 1px solid;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.todo-time {
  font-size: 12px;
  color: var(--text-muted);
}

.delete-btn {
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
  flex-shrink: 0;
  opacity: 0;
  transition: all 0.15s;
}

.todo-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.show-completed-bar {
  text-align: center;
  padding: 12px;
}

.show-completed-bar button {
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  padding: 6px 16px;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
}

.show-completed-bar button:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
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
  color: var(--text-muted);
}

/* 列表动画 */
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateX(-20px);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(20px);
}
</style>
