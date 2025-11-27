<script setup lang="ts">
import { ref, watch } from 'vue';

interface Props {
  modelValue: string;
  placeholder?: string;
  autofocus?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '搜索...',
  autofocus: false
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  search: [value: string];
  clear: [];
}>();

const inputRef = ref<HTMLInputElement>();
const localValue = ref(props.modelValue);

watch(() => props.modelValue, (newVal) => {
  localValue.value = newVal;
});

watch(localValue, (newVal) => {
  emit('update:modelValue', newVal);
  emit('search', newVal);
});

function clear() {
  localValue.value = '';
  emit('clear');
  inputRef.value?.focus();
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    clear();
  }
}

defineExpose({
  focus: () => inputRef.value?.focus()
});
</script>

<template>
  <div class="search-bar">
    <div class="search-icon">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.35-4.35"></path>
      </svg>
    </div>
    <input
      ref="inputRef"
      v-model="localValue"
      type="text"
      class="search-input"
      :placeholder="placeholder"
      :autofocus="autofocus"
      @keydown="handleKeydown"
    />
    <button
      v-if="localValue"
      class="clear-button"
      @click="clear"
      title="清除"
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.search-bar {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
  max-width: 400px;
}

.search-icon {
  position: absolute;
  left: 12px;
  color: var(--text-secondary);
  pointer-events: none;
  display: flex;
  align-items: center;
}

.search-input {
  width: 100%;
  padding: 8px 36px 8px 36px;
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.search-input:focus {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.search-input::placeholder {
  color: var(--text-secondary);
}

.clear-button {
  position: absolute;
  right: 8px;
  padding: 4px;
  border: none;
  background: none;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.15s, color 0.15s;
}

.clear-button:hover {
  background-color: var(--bg-secondary);
  color: var(--text-primary);
}
</style>
