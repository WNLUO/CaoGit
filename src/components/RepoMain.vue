<script setup lang="ts">
import type { Repository } from '../types';
import ChangesView from './ChangesView.vue';
import HistoryView from './HistoryView.vue';
import DiffView from './DiffView.vue';

defineProps<{
  repo: Repository;
}>();
</script>

<template>
  <div class="repo-main">
    <!-- Left Panel: Changes & Commit -->
    <div class="left-panel">
      <ChangesView />
    </div>

    <!-- Right Panel: Diff & History -->
    <div class="right-panel">
      <div class="top-pane">
        <DiffView />
      </div>
      <div class="bottom-pane">
        <div class="pane-header">提交历史 (HISTORY)</div>
        <HistoryView />
      </div>
    </div>
  </div>
</template>

<style scoped>
.repo-main {
  display: flex;
  flex: 1;
  height: 100%;
  overflow: hidden;
}

.left-panel {
  width: 320px;
  min-width: 250px;
  max-width: 400px;
  height: 100%;
  border-right: 1px solid var(--border-color);
}

.right-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.top-pane {
  flex: 1;
  min-height: 0;
  border-bottom: 1px solid var(--border-color);
}

.bottom-pane {
  height: 300px; /* Fixed height for history as per typical layout, or could be flex */
  min-height: 150px;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
}

.pane-header {
  padding: var(--spacing-xs) var(--spacing-md);
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}
</style>
