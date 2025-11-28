<script setup lang="ts">
import type { DiffStats } from '../../types';

interface Props {
  stats: DiffStats;
  compact?: boolean;
}

withDefaults(defineProps<Props>(), {
  compact: false
});
</script>

<template>
  <div :class="['diff-stats', { compact }]">
    <span class="stat additions">+{{ stats.additions }}</span>
    <span class="stat deletions">-{{ stats.deletions }}</span>
    <div class="stat-bar" v-if="!compact">
      <div
        class="bar-segment additions"
        :style="{ width: (stats.additions / stats.total * 100) + '%' }"
      ></div>
      <div
        class="bar-segment deletions"
        :style="{ width: (stats.deletions / stats.total * 100) + '%' }"
      ></div>
    </div>
  </div>
</template>

<style scoped>
.diff-stats {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  font-weight: 500;
  font-family: ui-monospace, 'SF Mono', monospace;
}

.diff-stats.compact {
  gap: 4px;
}

.stat {
  display: inline-flex;
  align-items: center;
  padding: 2px 6px;
  border-radius: 4px;
  line-height: 1;
}

.stat.additions {
  color: #10b981;
  background-color: rgba(16, 185, 129, 0.1);
}

.stat.deletions {
  color: #ef4444;
  background-color: rgba(239, 68, 68, 0.1);
}

.stat-bar {
  display: flex;
  height: 4px;
  border-radius: 2px;
  overflow: hidden;
  flex: 1;
  max-width: 100px;
  background-color: var(--bg-tertiary);
}

.bar-segment {
  height: 100%;
  transition: width 0.3s ease;
}

.bar-segment.additions {
  background-color: #10b981;
}

.bar-segment.deletions {
  background-color: #ef4444;
}
</style>
