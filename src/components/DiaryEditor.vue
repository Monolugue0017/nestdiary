<script setup lang="ts">
import { useEditor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import Placeholder from "@tiptap/extension-placeholder";
import Typography from "@tiptap/extension-typography";
import { watch, onBeforeUnmount, ref } from "vue";

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
  "update:plainText": [value: string];
  "update:wordCount": [count: number];
}>();

const isFocused = ref(false);

const editor = useEditor({
  content: props.modelValue || "",
  extensions: [
    StarterKit.configure({
      heading: { levels: [1, 2, 3] },
    }),
    Placeholder.configure({
      placeholder: props.placeholder || "开始写下今天的想法...",
    }),
    Typography,
  ],
  editorProps: {
    attributes: {
      class: "diary-editor",
      spellcheck: "false",
    },
  },
  onUpdate: ({ editor }) => {
    const html = editor.getHTML();
    const text = editor.getText();
    emit("update:modelValue", html);
    emit("update:plainText", text);
    emit("update:wordCount", text.length);
  },
  onFocus: () => {
    isFocused.value = true;
  },
  onBlur: () => {
    isFocused.value = false;
  },
});

// 外部值变化时更新编辑器
watch(
  () => props.modelValue,
  (newValue) => {
    if (editor.value && newValue !== editor.value.getHTML()) {
      editor.value.commands.setContent(newValue || "", false);
    }
  }
);

onBeforeUnmount(() => {
  editor.value?.destroy();
});

// 工具栏操作
function setHeading(level: 1 | 2 | 3) {
  editor.value?.chain().focus().toggleHeading({ level }).run();
}
function toggleBold() {
  editor.value?.chain().focus().toggleBold().run();
}
function toggleItalic() {
  editor.value?.chain().focus().toggleItalic().run();
}
function toggleBulletList() {
  editor.value?.chain().focus().toggleBulletList().run();
}
function toggleOrderedList() {
  editor.value?.chain().focus().toggleOrderedList().run();
}
function toggleBlockquote() {
  editor.value?.chain().focus().toggleBlockquote().run();
}
function toggleCodeBlock() {
  editor.value?.chain().focus().toggleCodeBlock().run();
}
</script>

<template>
  <div class="editor-wrapper" :class="{ focused: isFocused }">
    <!-- 工具栏 -->
    <div class="editor-toolbar" v-if="editor">
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('heading', { level: 1 }) }"
        title="标题1"
        @click="setHeading(1)"
      >
        H1
      </button>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('heading', { level: 2 }) }"
        title="标题2"
        @click="setHeading(2)"
      >
        H2
      </button>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('heading', { level: 3 }) }"
        title="标题3"
        @click="setHeading(3)"
      >
        H3
      </button>
      <span class="divider"></span>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('bold') }"
        title="粗体 (Ctrl+B)"
        @click="toggleBold"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
          <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
        </svg>
      </button>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('italic') }"
        title="斜体 (Ctrl+I)"
        @click="toggleItalic"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="19" y1="4" x2="10" y2="4" />
          <line x1="14" y1="20" x2="5" y2="20" />
          <line x1="15" y1="4" x2="9" y2="20" />
        </svg>
      </button>
      <span class="divider"></span>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('bulletList') }"
        title="无序列表"
        @click="toggleBulletList"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="8" y1="6" x2="21" y2="6" />
          <line x1="8" y1="12" x2="21" y2="12" />
          <line x1="8" y1="18" x2="21" y2="18" />
          <line x1="3" y1="6" x2="3.01" y2="6" />
          <line x1="3" y1="12" x2="3.01" y2="12" />
          <line x1="3" y1="18" x2="3.01" y2="18" />
        </svg>
      </button>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('orderedList') }"
        title="有序列表"
        @click="toggleOrderedList"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="10" y1="6" x2="21" y2="6" />
          <line x1="10" y1="12" x2="21" y2="12" />
          <line x1="10" y1="18" x2="21" y2="18" />
          <path d="M4 6h1v4M4 10h2M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" />
        </svg>
      </button>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('blockquote') }"
        title="引用"
        @click="toggleBlockquote"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z" />
          <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z" />
        </svg>
      </button>
      <button
        class="tool-btn"
        :class="{ active: editor.isActive('codeBlock') }"
        title="代码块"
        @click="toggleCodeBlock"
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="16 18 22 12 16 6" />
          <polyline points="8 6 2 12 8 18" />
        </svg>
      </button>
    </div>

    <!-- 编辑器内容区 -->
    <EditorContent :editor="editor" class="editor-content" />
  </div>
</template>

<style scoped>
.editor-wrapper {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  overflow: hidden;
  transition: border-color 0.2s;
}

.editor-wrapper.focused {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-card);
  flex-wrap: wrap;
}

.tool-btn {
  width: 30px;
  height: 30px;
  border: none;
  background: transparent;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
  transition: all 0.15s;
}

.tool-btn:hover {
  background: var(--bg-sidebar);
  color: var(--text-primary);
}

.tool-btn.active {
  background: var(--primary-color);
  color: white;
}

.divider {
  width: 1px;
  height: 20px;
  background: var(--border-color);
  margin: 0 4px;
}

.editor-content {
  padding: 16px 20px;
  min-height: 400px;
}

.editor-content :deep(.ProseMirror) {
  outline: none;
  min-height: 350px;
}
</style>
