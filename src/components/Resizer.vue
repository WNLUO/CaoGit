<template>
  <div
    :class="['resizer', `resizer-${direction}`, { 'is-dragging': isDragging }]"
    @mousedown="startResize"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  direction: 'horizontal' | 'vertical'
}>()

const emit = defineEmits<{
  resize: [delta: number]
}>()

const isDragging = ref(false)

function startResize(e: MouseEvent) {
  e.preventDefault()

  const startPos = props.direction === 'horizontal' ? e.clientX : e.clientY
  let lastPos = startPos

  // 设置拖动状态
  isDragging.value = true

  // 添加拖动时的视觉反馈
  document.body.style.cursor = props.direction === 'horizontal' ? 'col-resize' : 'row-resize'
  document.body.style.userSelect = 'none'

  const handleMouseMove = (moveEvent: MouseEvent) => {
    const currentPos = props.direction === 'horizontal' ? moveEvent.clientX : moveEvent.clientY
    const delta = currentPos - lastPos

    // 只有当有实际移动时才触发事件
    if (delta !== 0) {
      emit('resize', delta)
      lastPos = currentPos
    }
  }

  const handleMouseUp = () => {
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', handleMouseUp)

    // 恢复默认样式
    document.body.style.cursor = ''
    document.body.style.userSelect = ''

    // 清除拖动状态
    isDragging.value = false
  }

  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseup', handleMouseUp)
}
</script>

<style scoped>
.resizer {
  position: relative;
  background: transparent;
  user-select: none;
  -webkit-user-select: none;
}

.resizer-horizontal {
  width: 4px;
  height: 100%;
  cursor: col-resize;
  background-color: transparent;
  transition: background-color 0.15s ease;
  position: relative;
}

.resizer-horizontal::before {
  content: '';
  position: absolute;
  top: 0;
  left: -2px;
  right: -2px;
  bottom: 0;
}

.resizer-horizontal:hover {
  background-color: var(--border-color);
}

.resizer-horizontal.is-dragging {
  background-color: var(--accent-color);
}

.resizer-vertical {
  width: 100%;
  height: 4px;
  cursor: row-resize;
  background-color: transparent;
  transition: background-color 0.15s ease;
  position: relative;
}

.resizer-vertical::before {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  top: -2px;
  bottom: -2px;
}

.resizer-vertical:hover {
  background-color: var(--border-color);
}

.resizer-vertical.is-dragging {
  background-color: var(--accent-color);
}
</style>
