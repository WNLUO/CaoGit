<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

interface MenuItem {
  label: string;
  icon?: string;
  action: () => void;
  danger?: boolean;
}

interface Props {
  items: MenuItem[];
  x: number;
  y: number;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'close'): void;
}>();

const menuRef = ref<HTMLElement | null>(null);

function handleClickOutside(event: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(event.target as Node)) {
    emit('close');
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});

function handleItemClick(item: MenuItem) {
  item.action();
  emit('close');
}
</script>

<template>
  <div
    ref="menuRef"
    class="context-menu"
    :style="{ left: `${x}px`, top: `${y}px` }"
  >
    <ul>
      <li
        v-for="(item, index) in items"
        :key="index"
        :class="{ danger: item.danger }"
        @click="handleItemClick(item)"
      >
        <span v-if="item.icon" class="menu-icon">{{ item.icon }}</span>
        <span class="menu-label">{{ item.label }}</span>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.context-menu {
  position: fixed;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-width: 180px;
  z-index: 9999;
  padding: 4px 0;
}

ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

li {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  transition: background-color var(--transition-fast);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

li:hover {
  background-color: var(--bg-hover);
}

li.danger {
  color: #ef4444;
}

li.danger:hover {
  background-color: rgba(239, 68, 68, 0.1);
}

.menu-icon {
  font-size: 14px;
  width: 16px;
  text-align: center;
}

.menu-label {
  flex: 1;
}
</style>
