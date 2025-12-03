<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { repoStore } from '../stores/repoStore';

const canvas = ref<HTMLCanvasElement | null>(null);
const container = ref<HTMLDivElement | null>(null);

interface GraphNode {
  commit: any;
  x: number;
  y: number;
  lane: number;
  color: string;
}

const colors = [
  '#3b82f6', // blue-500
  '#10b981', // emerald-500
  '#f59e0b', // amber-500
  '#ef4444', // red-500
  '#8b5cf6', // violet-500
  '#ec4899', // pink-500
  '#06b6d4', // cyan-500
];

const nodes = computed<GraphNode[]>(() => {
  const result: GraphNode[] = [];
  const commits = repoStore.commits;

  commits.forEach((commit, index) => {
    result.push({
      commit,
      x: 30,
      y: index * 50 + 30,
      lane: index % 3,
      color: colors[index % colors.length],
    });
  });

  return result;
});

function drawGraph() {
  if (!canvas.value) return;

  const ctx = canvas.value.getContext('2d');
  if (!ctx) return;

  const width = canvas.value.width;
  const height = canvas.value.height;

  // Clear canvas
  ctx.clearRect(0, 0, width, height);

  // Draw connections
  for (let i = 0; i < nodes.value.length - 1; i++) {
    const node = nodes.value[i];
    const nextNode = nodes.value[i + 1];

    ctx.strokeStyle = node.color;
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(node.x, node.y);
    ctx.lineTo(nextNode.x, nextNode.y);
    ctx.stroke();
  }

  // Draw nodes
  nodes.value.forEach(node => {
    ctx.fillStyle = node.color;
    ctx.beginPath();
    ctx.arc(node.x, node.y, 6, 0, 2 * Math.PI);
    ctx.fill();

    ctx.strokeStyle = '#fff';
    ctx.lineWidth = 2;
    ctx.stroke();
  });
}

function resizeCanvas() {
  if (!canvas.value || !container.value) return;

  canvas.value.width = container.value.clientWidth;
  canvas.value.height = Math.max(nodes.value.length * 50 + 60, container.value.clientHeight);

  drawGraph();
}

onMounted(() => {
  resizeCanvas();
  window.addEventListener('resize', resizeCanvas);
});

watch(() => repoStore.commits, () => {
  resizeCanvas();
}, { deep: true });
</script>

<template>
  <div ref="container" class="branch-graph">
    <canvas ref="canvas"></canvas>
  </div>
</template>

<style scoped>
.branch-graph {
  width: 100%;
  height: 100%;
  overflow: auto;
  background-color: transparent;
}

canvas {
  display: block;
}
</style>
