<script setup lang="ts">
import Sidebar from './Sidebar.vue';
import TopBar from './TopBar.vue';
</script>

<template>
  <div class="app-layout">
    <Sidebar />
    <main class="main-content">
      <TopBar />
      <div class="content-area">
        <slot></slot>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-lg);
}
</style>
