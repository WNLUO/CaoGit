<script setup lang="ts">
import { repoStore } from '../stores/repoStore';

function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
}
</script>

<template>
  <div class="history-view">
    <table class="commit-table">
      <thead>
        <tr>
          <th class="col-graph">Graph</th>
          <th class="col-message">提交信息 (Message)</th>
          <th class="col-author">作者 (Author)</th>
          <th class="col-date">日期 (Date)</th>
          <th class="col-hash">Hash</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="commit in repoStore.commits" :key="commit.hash">
          <td class="col-graph">
            <span class="graph-node">●</span>
          </td>
          <td class="col-message">{{ commit.message }}</td>
          <td class="col-author">{{ commit.author }}</td>
          <td class="col-date">{{ formatDate(commit.date) }}</td>
          <td class="col-hash">{{ commit.hash.substring(0, 7) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.history-view {
  height: 100%;
  overflow-y: auto;
  background-color: var(--bg-primary);
}

.commit-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--font-size-sm);
}

th {
  text-align: left;
  padding: var(--spacing-sm);
  border-bottom: 1px solid var(--border-color);
  color: var(--text-secondary);
  font-weight: 600;
  background-color: var(--bg-secondary);
  position: sticky;
  top: 0;
}

td {
  padding: var(--spacing-sm);
  border-bottom: 1px solid var(--border-color);
  color: var(--text-primary);
}

tr:hover {
  background-color: var(--bg-secondary);
}

.col-graph {
  width: 50px;
  text-align: center;
  color: var(--accent-color);
  position: relative;
}

.graph-node {
  position: relative;
  z-index: 2;
  background-color: var(--bg-primary);
  border-radius: 50%;
}

.col-graph::before {
  content: '';
  position: absolute;
  top: 0;
  bottom: 0;
  left: 50%;
  width: 2px;
  background-color: var(--border-color);
  transform: translateX(-50%);
  z-index: 1;
}

tr:first-child .col-graph::before {
  top: 50%;
}

tr:last-child .col-graph::before {
  bottom: 50%;
}

.col-message {
  font-weight: 500;
}

.col-author, .col-date, .col-hash {
  color: var(--text-secondary);
  width: 150px;
}

.col-hash {
  font-family: monospace;
  width: 100px;
}
</style>
