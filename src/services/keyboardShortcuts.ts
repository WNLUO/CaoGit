/**
 * Keyboard Shortcuts Service
 *
 * Provides global keyboard shortcut handling for the application.
 */

import { ref, onMounted, onUnmounted } from 'vue';

export interface Shortcut {
  key: string;
  ctrl?: boolean;
  alt?: boolean;
  shift?: boolean;
  meta?: boolean; // Command key on Mac
  description: string;
  action: () => void;
}

type ShortcutMap = Map<string, Shortcut>;

const shortcuts: ShortcutMap = new Map();
const isEnabled = ref(true);

/**
 * Generate a unique key for a shortcut
 */
function getShortcutKey(shortcut: Omit<Shortcut, 'description' | 'action'>): string {
  const parts: string[] = [];
  if (shortcut.ctrl) parts.push('ctrl');
  if (shortcut.alt) parts.push('alt');
  if (shortcut.shift) parts.push('shift');
  if (shortcut.meta) parts.push('meta');
  parts.push(shortcut.key.toLowerCase());
  return parts.join('+');
}

/**
 * Check if a keyboard event matches a shortcut
 */
function matchesShortcut(event: KeyboardEvent, shortcut: Shortcut): boolean {
  return (
    event.key.toLowerCase() === shortcut.key.toLowerCase() &&
    !!event.ctrlKey === !!shortcut.ctrl &&
    !!event.altKey === !!shortcut.alt &&
    !!event.shiftKey === !!shortcut.shift &&
    !!event.metaKey === !!shortcut.meta
  );
}

/**
 * Handle keyboard events
 */
function handleKeydown(event: KeyboardEvent): void {
  if (!isEnabled.value) return;

  // Ignore events from input elements
  const target = event.target as HTMLElement;
  if (
    target.tagName === 'INPUT' ||
    target.tagName === 'TEXTAREA' ||
    target.isContentEditable
  ) {
    return;
  }

  for (const shortcut of shortcuts.values()) {
    if (matchesShortcut(event, shortcut)) {
      event.preventDefault();
      event.stopPropagation();
      shortcut.action();
      return;
    }
  }
}

/**
 * Register a keyboard shortcut
 */
export function registerShortcut(shortcut: Shortcut): () => void {
  const key = getShortcutKey(shortcut);
  shortcuts.set(key, shortcut);

  // Return unregister function
  return () => {
    shortcuts.delete(key);
  };
}

/**
 * Unregister a keyboard shortcut
 */
export function unregisterShortcut(shortcut: Omit<Shortcut, 'description' | 'action'>): void {
  const key = getShortcutKey(shortcut);
  shortcuts.delete(key);
}

/**
 * Enable keyboard shortcuts
 */
export function enableShortcuts(): void {
  isEnabled.value = true;
}

/**
 * Disable keyboard shortcuts
 */
export function disableShortcuts(): void {
  isEnabled.value = false;
}

/**
 * Get all registered shortcuts
 */
export function getShortcuts(): Shortcut[] {
  return Array.from(shortcuts.values());
}

/**
 * Format a shortcut for display
 */
export function formatShortcut(shortcut: Omit<Shortcut, 'description' | 'action'>): string {
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
  const parts: string[] = [];

  if (shortcut.ctrl) parts.push(isMac ? '⌃' : 'Ctrl');
  if (shortcut.alt) parts.push(isMac ? '⌥' : 'Alt');
  if (shortcut.shift) parts.push(isMac ? '⇧' : 'Shift');
  if (shortcut.meta) parts.push(isMac ? '⌘' : 'Win');

  // Capitalize the key
  const key = shortcut.key.length === 1
    ? shortcut.key.toUpperCase()
    : shortcut.key.charAt(0).toUpperCase() + shortcut.key.slice(1);
  parts.push(key);

  return parts.join(isMac ? '' : '+');
}

/**
 * Composable for using keyboard shortcuts in Vue components
 */
export function useKeyboardShortcuts() {
  onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
  });

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown);
  });

  return {
    registerShortcut,
    unregisterShortcut,
    enableShortcuts,
    disableShortcuts,
    getShortcuts,
    formatShortcut,
    isEnabled,
  };
}

// Default shortcuts
export const DEFAULT_SHORTCUTS: Omit<Shortcut, 'action'>[] = [
  { key: 's', ctrl: true, description: '保存/提交' },
  { key: 'r', ctrl: true, description: '刷新' },
  { key: 'p', ctrl: true, shift: true, description: '推送' },
  { key: 'f', ctrl: true, description: '搜索' },
  { key: 'b', ctrl: true, description: '切换分支' },
  { key: ',', ctrl: true, description: '打开设置' },
  { key: 'Escape', description: '关闭对话框' },
];

/**
 * Initialize global keyboard shortcuts
 */
export function initKeyboardShortcuts(): void {
  window.addEventListener('keydown', handleKeydown);
}

/**
 * Cleanup global keyboard shortcuts
 */
export function cleanupKeyboardShortcuts(): void {
  window.removeEventListener('keydown', handleKeydown);
}
