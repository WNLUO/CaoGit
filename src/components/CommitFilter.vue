<script setup lang="ts">
import { ref, computed } from 'vue';
import SearchBar from './SearchBar.vue';

export interface FilterOptions {
  searchText: string;
  author: string;
  dateFrom: string;
  dateTo: string;
  branch: string;
}

interface Props {
  branches?: string[];
}

withDefaults(defineProps<Props>(), {
  branches: () => []
});

const emit = defineEmits<{
  filter: [options: FilterOptions];
}>();

const isExpanded = ref(false);
const searchText = ref('');
const author = ref('');
const dateFrom = ref('');
const dateTo = ref('');
const selectedBranch = ref('');

const hasActiveFilters = computed(() => {
  return !!(author.value || dateFrom.value || dateTo.value || selectedBranch.value);
});

function applyFilter() {
  emit('filter', {
    searchText: searchText.value,
    author: author.value,
    dateFrom: dateFrom.value,
    dateTo: dateTo.value,
    branch: selectedBranch.value
  });
}

function clearFilters() {
  author.value = '';
  dateFrom.value = '';
  dateTo.value = '';
  selectedBranch.value = '';
  applyFilter();
}

function handleSearch(value: string) {
  searchText.value = value;
  applyFilter();
}
</script>

<template>
  <div class="commit-filter">
    <div class="filter-header">
      <div class="search-section">
        <SearchBar
          v-model="searchText"
          placeholder="搜索提交消息、哈希..."
          @search="handleSearch"
        />
      </div>

      <button
        class="filter-toggle"
        :class="{ active: isExpanded || hasActiveFilters }"
        @click="isExpanded = !isExpanded"
        :title="isExpanded ? '隐藏过滤器' : '显示过滤器'"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
        </svg>
        <span v-if="hasActiveFilters" class="filter-badge">{{ Object.values({ author, dateFrom, dateTo, selectedBranch }).filter(v => v).length }}</span>
      </button>
    </div>

    <div v-if="isExpanded" class="filter-panel">
      <div class="filter-row">
        <div class="filter-field">
          <label>作者</label>
          <input
            v-model="author"
            type="text"
            placeholder="输入作者名称..."
            @input="applyFilter"
          />
        </div>

        <div class="filter-field">
          <label>分支</label>
          <select v-model="selectedBranch" @change="applyFilter">
            <option value="">所有分支</option>
            <option v-for="branch in branches" :key="branch" :value="branch">
              {{ branch }}
            </option>
          </select>
        </div>
      </div>

      <div class="filter-row">
        <div class="filter-field">
          <label>开始日期</label>
          <input
            v-model="dateFrom"
            type="date"
            @change="applyFilter"
          />
        </div>

        <div class="filter-field">
          <label>结束日期</label>
          <input
            v-model="dateTo"
            type="date"
            @change="applyFilter"
          />
        </div>
      </div>

      <div class="filter-actions">
        <button class="clear-button" @click="clearFilters">
          清除所有过滤器
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.commit-filter {
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.filter-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
}

.search-section {
  flex: 1;
}

.filter-toggle {
  position: relative;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: all 0.15s;
}

.filter-toggle:hover {
  background-color: var(--bg-secondary);
  border-color: var(--accent-color);
  color: var(--text-primary);
}

.filter-toggle.active {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
  color: white;
}

.filter-badge {
  position: absolute;
  top: -6px;
  right: -6px;
  background-color: #ef4444;
  color: white;
  font-size: 11px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 18px;
  text-align: center;
}

.filter-panel {
  padding: 0 12px 12px;
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

.filter-row {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.filter-field {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.filter-field label {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-secondary);
}

.filter-field input,
.filter-field select {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
  transition: border-color 0.15s;
}

.filter-field input:focus,
.filter-field select:focus {
  border-color: var(--accent-color);
}

.filter-actions {
  display: flex;
  justify-content: flex-end;
}

.clear-button {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.clear-button:hover {
  background-color: var(--bg-secondary);
  border-color: var(--accent-color);
  color: var(--text-primary);
}
</style>
