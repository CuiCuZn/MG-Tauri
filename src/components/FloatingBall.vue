<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const buttons = [
  { id: 1, label: "网页", url: "https://www.baidu.com", color: "#3385ff" },
  { id: 2, label: "图片", url: "https://image.baidu.com/search/index?tn=baiduimage", color: "#ff6b6b" },
  { id: 3, label: "视频", url: "https://v.baidu.com/", color: "#ffa500" },
  { id: 4, label: "新闻", url: "https://news.baidu.com/", color: "#4ecdc4" },
  { id: 5, label: "天气", url: "https://weather.baidu.com/", color: "#95e1d3" },
];

const isDragging = ref(false);
const dragOffset = ref({ x: 0, y: 0 });

async function handleButtonClick(url: string) {
  try {
    await invoke("navigate_content_window", { url });
  } catch (e) {
    console.error("Failed to navigate:", e);
  }
}

async function startDrag(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains('toolbar-button')) {
    return;
  }
  
  isDragging.value = true;
  dragOffset.value = { x: e.clientX, y: e.clientY };
  
  try {
    const appWindow = getCurrentWindow();
    await appWindow.startDragging();
  } catch (err) {
    console.error("Drag error:", err);
  }
}

function onDragEnd() {
  isDragging.value = false;
}
</script>

<template>
  <div 
    class="floating-bar" 
    @mousedown="startDrag"
    @mouseup="onDragEnd"
  >
    <button
      v-for="btn in buttons"
      :key="btn.id"
      class="toolbar-button"
      :style="{ backgroundColor: btn.color }"
      @click="handleButtonClick(btn.url)"
    >
      {{ btn.label }}
    </button>
  </div>
</template>

<style scoped>
.floating-bar {
  width: 320px;
  height: 40px;
  border-radius: 8px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 4px 8px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
  gap: 8px;
  cursor: grab;
  user-select: none;
}

.floating-bar:active {
  cursor: grabbing;
}

.toolbar-button {
  flex: 1;
  height: 32px;
  border-radius: 6px;
  border: none;
  color: white;
  font-size: 13px;
  font-weight: 500;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  transition: transform 0.15s ease, filter 0.15s ease;
  padding: 0 8px;
}

.toolbar-button:hover {
  transform: scale(1.05);
  filter: brightness(1.1);
}

.toolbar-button:active {
  transform: scale(0.95);
}
</style>
