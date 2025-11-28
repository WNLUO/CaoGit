/**
 * Internationalization (i18n) Module
 *
 * Provides multi-language support for the application.
 */

import { ref, computed } from 'vue';
import zhCN from './locales/zh-CN';
import enUS from './locales/en-US';

export type Locale = 'zh-CN' | 'en-US';

export interface LocaleMessages {
  [key: string]: string | LocaleMessages;
}

const locales: Record<Locale, LocaleMessages> = {
  'zh-CN': zhCN,
  'en-US': enUS,
};

// Current locale state
const currentLocale = ref<Locale>('zh-CN');

/**
 * Set the current locale
 */
export function setLocale(locale: Locale): void {
  if (locales[locale]) {
    currentLocale.value = locale;
    localStorage.setItem('locale', locale);
  }
}

/**
 * Get the current locale
 */
export function getLocale(): Locale {
  return currentLocale.value;
}

/**
 * Initialize locale from localStorage or browser settings
 */
export function initLocale(): void {
  const saved = localStorage.getItem('locale') as Locale | null;
  if (saved && locales[saved]) {
    currentLocale.value = saved;
  } else {
    // Detect browser language
    const browserLang = navigator.language;
    if (browserLang.startsWith('zh')) {
      currentLocale.value = 'zh-CN';
    } else {
      currentLocale.value = 'en-US';
    }
  }
}

/**
 * Get a translated message by key path
 * @param key - Dot-separated key path (e.g., 'common.save')
 * @param params - Optional parameters for interpolation
 */
export function t(key: string, params?: Record<string, string | number>): string {
  const messages = locales[currentLocale.value];
  const keys = key.split('.');
  let result: string | LocaleMessages = messages;

  for (const k of keys) {
    if (typeof result === 'object' && result !== null && k in result) {
      result = result[k];
    } else {
      // Key not found, return the key itself
      return key;
    }
  }

  if (typeof result !== 'string') {
    return key;
  }

  // Handle parameter interpolation
  if (params) {
    return result.replace(/\{(\w+)\}/g, (_, paramKey) => {
      return params[paramKey]?.toString() || `{${paramKey}}`;
    });
  }

  return result;
}

/**
 * Composable for using i18n in Vue components
 */
export function useI18n() {
  return {
    locale: computed(() => currentLocale.value),
    setLocale,
    t,
    availableLocales: Object.keys(locales) as Locale[],
  };
}
