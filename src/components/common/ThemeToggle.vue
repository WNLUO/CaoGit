<script setup lang="ts">
import { computed } from 'vue';
import { themeStore, type Theme } from '../../stores/themeStore';
import { settingsStore } from '../../stores/settingsStore';

const currentTheme = computed(() => settingsStore.settings.appearance.theme);
const effectiveTheme = computed(() => themeStore.effectiveTheme);

const showDropdown = defineModel<boolean>('showDropdown', { default: false });

function handleToggle() {
  // Toggle logic: if auto/system, switch to opposite of effective. Else toggle light/dark.
  const current = currentTheme.value;
  const effective = effectiveTheme.value;
  let nextTheme: Theme = 'light';

  if (current === 'system') {
    nextTheme = effective === 'dark' ? 'light' : 'dark';
  } else {
    nextTheme = current === 'dark' ? 'light' : 'dark';
  }
  
  settingsStore.updateAppearance({ theme: nextTheme });
}

function selectTheme(theme: Theme) {
  // Use 'system' for consistency with settings interface
  const valueToSave = theme === 'auto' ? 'system' : theme;
  settingsStore.updateAppearance({ theme: valueToSave });
  showDropdown.value = false;
}
</script>

<template>
  <div class="theme-toggle">
    <button
      class="theme-button"
      @click="handleToggle"
      :title="`当前主题: ${currentTheme === 'system' ? '自动' : currentTheme === 'dark' ? '暗黑' : '明亮'}`"
    >
      <!-- Sun icon for light theme -->
      <svg
        v-if="effectiveTheme === 'light'"
        width="18"
        height="18"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <circle cx="12" cy="12" r="5"></circle>
        <line x1="12" y1="1" x2="12" y2="3"></line>
        <line x1="12" y1="21" x2="12" y2="23"></line>
        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
        <line x1="1" y1="12" x2="3" y2="12"></line>
        <line x1="21" y1="12" x2="23" y2="12"></line>
        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
      </svg>

      <!-- Moon icon for dark theme -->
      <svg
        v-else
        width="18"
        height="18"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
      </svg>
    </button>

    <!-- Dropdown menu -->
    <div v-if="showDropdown" class="theme-dropdown">
      <button
        class="theme-option"
        :class="{ active: currentTheme === 'light' }"
        @click="selectTheme('light')"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5"></circle>
          <line x1="12" y1="1" x2="12" y2="3"></line>
          <line x1="12" y1="21" x2="12" y2="23"></line>
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
          <line x1="1" y1="12" x2="3" y2="12"></line>
          <line x1="21" y1="12" x2="23" y2="12"></line>
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
        </svg>
        <span>明亮</span>
        <span v-if="currentTheme === 'light'" class="checkmark">✓</span>
      </button>

      <button
        class="theme-option"
        :class="{ active: currentTheme === 'dark' }"
        @click="selectTheme('dark')"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
        <span>暗黑</span>
        <span v-if="currentTheme === 'dark'" class="checkmark">✓</span>
      </button>

      <button
        class="theme-option"
        :class="{ active: currentTheme === 'system' }"
        @click="selectTheme('system')"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
          <line x1="8" y1="21" x2="16" y2="21"></line>
          <line x1="12" y1="17" x2="12" y2="21"></line>
        </svg>
        <span>自动</span>
        <span v-if="currentTheme === 'system'" class="checkmark">✓</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.theme-toggle {
  position: relative;
}

.theme-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  border-radius: var(--radius-md);
  background-color: transparent;
  color: var(--text-secondary);
  transition: all var(--transition-fast);
}

.theme-button:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.theme-button svg {
  transition: transform 0.3s ease;
}

.theme-button:hover svg {
  transform: rotate(20deg);
}

.theme-dropdown {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  min-width: 140px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
  z-index: 1000;
  animation: slideDown 0.2s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 14px;
  text-align: left;
  background-color: transparent;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  transition: background-color var(--transition-fast);
  border: none;
  cursor: pointer;
}

.theme-option:hover {
  background-color: var(--bg-hover);
}

.theme-option.active {
  background-color: var(--accent-subtle);
  color: var(--accent-color);
  font-weight: var(--font-weight-medium);
}

.theme-option svg {
  flex-shrink: 0;
}

.theme-option span {
  flex: 1;
}

.checkmark {
  margin-left: auto;
  color: var(--accent-color);
  font-weight: var(--font-weight-bold);
}
</style>
