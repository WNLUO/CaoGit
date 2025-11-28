/**
 * Unit tests for the i18n module
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { t, setLocale, getLocale } from '../../src/i18n';

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  clear: vi.fn(),
  removeItem: vi.fn(),
  length: 0,
  key: vi.fn(),
};
vi.stubGlobal('localStorage', localStorageMock);

describe('i18n', () => {
  beforeEach(() => {
    // Reset to default locale before each test
    vi.clearAllMocks();
    setLocale('zh-CN');
  });

  describe('t()', () => {
    it('should return Chinese translation by default', () => {
      expect(t('common.save')).toBe('保存');
    });

    it('should return English translation when locale is en-US', () => {
      setLocale('en-US');
      expect(t('common.save')).toBe('Save');
    });

    it('should return the key if translation not found', () => {
      expect(t('nonexistent.key')).toBe('nonexistent.key');
    });

    it('should handle nested keys', () => {
      expect(t('repository.status.online')).toBe('在线');
      setLocale('en-US');
      expect(t('repository.status.online')).toBe('Online');
    });
  });

  describe('setLocale() and getLocale()', () => {
    it('should change locale', () => {
      expect(getLocale()).toBe('zh-CN');
      setLocale('en-US');
      expect(getLocale()).toBe('en-US');
    });

    it('should not change locale for invalid value', () => {
      setLocale('invalid' as any);
      expect(getLocale()).toBe('zh-CN');
    });
  });
});
