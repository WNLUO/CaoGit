<script setup lang="ts">
import EnhancedDiffView from './EnhancedDiffView.vue';
</script>

<template>
  <div class="diff-view">
    <EnhancedDiffView />
  </div>
</template>

<style scoped>
.diff-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-primary);
}
</style>
