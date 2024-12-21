<script setup lang="ts">
import { onMounted, ref } from "vue";

import { AppStore } from "../lib/store";

const DEFAULT_IS_EDIT_MODE = true;

const isEditMode = ref(DEFAULT_IS_EDIT_MODE);
const isLoading = ref(true);

const handleModeChange = async () => {
  try {
    await AppStore.setItem("isEditMode", isEditMode.value);
  } catch (error) {
    console.error("Failed to save mode:", error);
  }
};

const loadStoredMode = async () => {
  try {
    const mode = await AppStore.getItem("isEditMode");

    isEditMode.value = Boolean(mode ?? DEFAULT_IS_EDIT_MODE);
    isLoading.value = false;
  } catch (error) {
    console.error("Failed to load mode:", error);
    isLoading.value = false;
  }
};

onMounted(() => {
  loadStoredMode();
});
</script>

<template>
  <div v-if="isLoading" class="text-gray-500">Loading...</div>
  <div v-else class="mode-toggle">
    <label class="switch">
      <input type="checkbox" v-model="isEditMode" @change="handleModeChange" />
      <span class="slider"></span>
    </label>
    <span class="mode-label">{{ isEditMode ? "Edit Mode" : "Run Mode" }}</span>
  </div>
</template>

<style scoped>
.mode-toggle {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.mode-label {
  font-weight: 500;
}

.switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 34px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.4s;
  border-radius: 34px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 26px;
  width: 26px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: 0.4s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #396cd8;
}

input:focus + .slider {
  box-shadow: 0 0 1px #396cd8;
}

input:checked + .slider:before {
  transform: translateX(26px);
}
</style>
