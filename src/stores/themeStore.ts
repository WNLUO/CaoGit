import { reactive } from 'vue';
import { GitApi } from '../services/gitApi';

export type Theme = 'light' | 'dark' | 'auto';

interface ThemeState {
  current: Theme;
  effectiveTheme: 'light' | 'dark';
}

// Load theme from localStorage
function loadTheme(): Theme {
  const saved = localStorage.getItem('theme');
  if (saved && ['light', 'dark', 'auto'].includes(saved)) {
    return saved as Theme;
  }
  return 'auto';
}

// Detect system theme
function getSystemTheme(): 'light' | 'dark' {
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  return 'light';
}

// Calculate effective theme
function calculateEffectiveTheme(theme: Theme): 'light' | 'dark' {
  if (theme === 'auto') {
    return getSystemTheme();
  }
  return theme;
}

export const themeStore = reactive<ThemeState>({
  current: loadTheme(),
  effectiveTheme: calculateEffectiveTheme(loadTheme()),
});

// Apply theme to document and window
export async function applyTheme(theme: 'light' | 'dark') {
  document.documentElement.setAttribute('data-theme', theme);
  themeStore.effectiveTheme = theme;

  // Update native window theme (for macOS titlebar)
  try {
    await GitApi.setWindowTheme(theme);
  } catch (error) {
    console.warn('Failed to set window theme:', error);
  }
}

// Set theme
export function setTheme(theme: Theme) {
  themeStore.current = theme;
  localStorage.setItem('theme', theme);
  applyTheme(calculateEffectiveTheme(theme));
}

// Toggle between light and dark
export function toggleTheme() {
  if (themeStore.current === 'auto') {
    // If auto, switch to opposite of current effective theme
    setTheme(themeStore.effectiveTheme === 'dark' ? 'light' : 'dark');
  } else {
    // Toggle between light and dark
    setTheme(themeStore.effectiveTheme === 'dark' ? 'light' : 'dark');
  }
}

// Watch for system theme changes
if (typeof window !== 'undefined' && window.matchMedia) {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

  // Modern browsers
  if (mediaQuery.addEventListener) {
    mediaQuery.addEventListener('change', (e) => {
      if (themeStore.current === 'auto') {
        applyTheme(e.matches ? 'dark' : 'light');
      }
    });
  }
  // Legacy browsers
  else if (mediaQuery.addListener) {
    mediaQuery.addListener((e) => {
      if (themeStore.current === 'auto') {
        applyTheme(e.matches ? 'dark' : 'light');
      }
    });
  }
}

// Initialize theme on load
applyTheme(calculateEffectiveTheme(themeStore.current));
