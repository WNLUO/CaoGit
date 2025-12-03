import { reactive } from 'vue';
import { GitApi } from '../services/gitApi';

export type Theme = 'light' | 'dark' | 'auto' | 'system';

interface ThemeState {
  current: Theme;
  effectiveTheme: 'light' | 'dark';
}

export const themeStore = reactive<ThemeState>({
  current: 'auto',
  effectiveTheme: 'light', // Will be updated by applyTheme
});

// Detect system theme
function getSystemTheme(): 'light' | 'dark' {
  if (typeof window !== 'undefined' && window.matchMedia) {
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const isLight = window.matchMedia('(prefers-color-scheme: light)').matches;
    console.log('[Theme] System theme detected - Dark:', isDark, 'Light:', isLight);

    if (isDark) return 'dark';
    if (isLight) return 'light';

    // Fallback
    return 'light';
  }
  console.log('[Theme] window.matchMedia not available, defaulting to light');
  return 'light';
}

// Calculate effective theme
function calculateEffectiveTheme(theme: Theme): 'light' | 'dark' {
  console.log('[Theme] Calculating effective theme for:', theme);
  if (theme === 'auto' || theme === 'system') {
    return getSystemTheme();
  }
  return theme as 'light' | 'dark';
}

// Apply theme to document and window
export async function applyTheme(theme: 'light' | 'dark') {
  console.log('[Theme] Applying theme:', theme);
  document.documentElement.setAttribute('data-theme', theme);
  themeStore.effectiveTheme = theme;

  // Update native window theme (for macOS titlebar)
  try {
    await GitApi.setWindowTheme(theme);
  } catch (error) {
    console.warn('Failed to set window theme:', error);
  }
}

// Set theme (Internal use or called by settingsStore)
export function setTheme(theme: Theme) {
  console.log('[Theme] Setting theme to:', theme);
  themeStore.current = theme;
  // localStorage.setItem('theme', theme); // Managed by settingsStore
  applyTheme(calculateEffectiveTheme(theme));
}

// Toggle between light and dark
export function toggleTheme() {
  // This function might need to be moved or updated to call settingsStore
  // For now, we'll leave it but it won't persist changes
  if (themeStore.current === 'auto' || themeStore.current === 'system') {
    setTheme(themeStore.effectiveTheme === 'dark' ? 'light' : 'dark');
  } else {
    setTheme(themeStore.effectiveTheme === 'dark' ? 'light' : 'dark');
  }
}

// Watch for system theme changes
if (typeof window !== 'undefined' && window.matchMedia) {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

  const handleThemeChange = (e: MediaQueryListEvent | MediaQueryList) => {
    if (themeStore.current === 'auto' || themeStore.current === 'system') {
      applyTheme(e.matches ? 'dark' : 'light');
    }
  };

  // Modern browsers
  if (mediaQuery.addEventListener) {
    mediaQuery.addEventListener('change', handleThemeChange);
  }
  // Legacy browsers
  else if (mediaQuery.addListener) {
    mediaQuery.addListener(handleThemeChange);
  }
}

// Initialize theme on load
applyTheme(calculateEffectiveTheme(themeStore.current));
