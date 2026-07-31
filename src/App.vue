<script setup lang="ts">
import { onMounted, onUnmounted, computed, watch } from "vue";
import { useAppStore } from "./stores/app";
import LockScreen from "./components/LockScreen.vue";
import Sidebar from "./components/Sidebar.vue";
import TopBar from "./components/TopBar.vue";

const appStore = useAppStore();

const showLockScreen = computed(() => {
  return appStore.hasPassword && !appStore.isUnlocked;
});

// 应用主题
watch(
  () => appStore.theme,
  (newTheme) => {
    document.documentElement.setAttribute("data-theme", newTheme);
  }
);

// 活动追踪
let activityTimer: number | null = null;

function handleActivity() {
  appStore.updateActivity();
}

onMounted(async () => {
  await appStore.init();
  document.documentElement.setAttribute("data-theme", appStore.theme);

  // 监听用户活动
  document.addEventListener("keydown", handleActivity);
  document.addEventListener("click", handleActivity);
  document.addEventListener("mousemove", () => {
    if (!activityTimer) {
      activityTimer = window.setTimeout(() => {
        handleActivity();
        activityTimer = null;
      }, 5000); // 每5秒最多记录一次mousemove
    }
  });
});

onUnmounted(() => {
  document.removeEventListener("keydown", handleActivity);
  document.removeEventListener("click", handleActivity);
  if (activityTimer) clearTimeout(activityTimer);
});

// 全局快捷键
function handleKeydown(e: KeyboardEvent) {
  if (showLockScreen.value) return;

  // Ctrl+1/2/3 切换模块
  if (e.ctrlKey && (e.key === "1" || e.key === "2" || e.key === "3")) {
    e.preventDefault();
    const routes = ["/diary", "/todo", "/memo"];
    const idx = parseInt(e.key) - 1;
    window.location.hash = `#${routes[idx]}`;
  }

  // Ctrl+K 打开搜索
  if (e.ctrlKey && e.key === "k") {
    e.preventDefault();
    window.location.hash = "#/search";
  }
}

onMounted(() => {
  document.addEventListener("keydown", handleKeydown);
});

onUnmounted(() => {
  document.removeEventListener("keydown", handleKeydown);
});
</script>

<template>
  <div class="app-container">
    <!-- 锁屏 -->
    <Transition name="fade">
      <LockScreen v-if="showLockScreen" />
    </Transition>

    <!-- 主界面 -->
    <div class="main-layout" v-show="!showLockScreen">
      <Sidebar />
      <div class="content-area">
        <TopBar />
        <main class="main-content">
          <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
              <component :is="Component" />
            </Transition>
          </RouterView>
        </main>
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  width: 100%;
  height: 100%;
}

.main-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}
</style>
