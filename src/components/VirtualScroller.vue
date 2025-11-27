<script setup lang="ts" generic="T">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';

interface Props {
  items: T[];
  itemHeight: number;
  visibleCount?: number;
  buffer?: number;
}

const props = withDefaults(defineProps<Props>(), {
  visibleCount: 20,
  buffer: 5
});

const emit = defineEmits<{
  loadMore: [];
}>();

defineSlots<{
  default(props: { item: T; index: number }): any;
}>();

const containerRef = ref<HTMLElement>();
const scrollTop = ref(0);
const containerHeight = ref(600);

// Calculate visible range
const startIndex = computed(() => {
  const index = Math.floor(scrollTop.value / props.itemHeight) - props.buffer;
  return Math.max(0, index);
});

const endIndex = computed(() => {
  const index = startIndex.value + props.visibleCount + props.buffer * 2;
  return Math.min(props.items.length, index);
});

const visibleItems = computed(() => {
  return props.items.slice(startIndex.value, endIndex.value).map((item, i) => ({
    item,
    index: startIndex.value + i,
  }));
});

const totalHeight = computed(() => props.items.length * props.itemHeight);
const offsetY = computed(() => startIndex.value * props.itemHeight);

// Handle scroll
function handleScroll(event: Event) {
  const target = event.target as HTMLElement;
  scrollTop.value = target.scrollTop;

  // Load more when near bottom
  const scrollBottom = target.scrollTop + target.clientHeight;
  const threshold = totalHeight.value - props.itemHeight * 10;
  if (scrollBottom >= threshold && endIndex.value >= props.items.length - 1) {
    emit('loadMore');
  }
}

// Update container height
function updateContainerHeight() {
  if (containerRef.value) {
    containerHeight.value = containerRef.value.clientHeight;
  }
}

onMounted(() => {
  updateContainerHeight();
  window.addEventListener('resize', updateContainerHeight);
});

onUnmounted(() => {
  window.removeEventListener('resize', updateContainerHeight);
});

// Watch items change and reset scroll if needed
watch(() => props.items.length, (newLen, oldLen) => {
  // If items were cleared, reset scroll
  if (newLen === 0 || newLen < oldLen / 2) {
    scrollTop.value = 0;
    if (containerRef.value) {
      containerRef.value.scrollTop = 0;
    }
  }
});
</script>

<template>
  <div
    ref="containerRef"
    class="virtual-scroller"
    @scroll="handleScroll"
  >
    <div
      class="virtual-scroller-content"
      :style="{ height: `${totalHeight}px` }"
    >
      <div
        class="virtual-scroller-items"
        :style="{ transform: `translateY(${offsetY}px)` }"
      >
        <div
          v-for="{ item, index } in visibleItems"
          :key="index"
          class="virtual-scroller-item"
          :style="{ height: `${itemHeight}px` }"
        >
          <slot :item="item" :index="index" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.virtual-scroller {
  overflow-y: auto;
  overflow-x: hidden;
  height: 100%;
  position: relative;
}

.virtual-scroller-content {
  position: relative;
  width: 100%;
}

.virtual-scroller-items {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
}

.virtual-scroller-item {
  overflow: hidden;
}
</style>
