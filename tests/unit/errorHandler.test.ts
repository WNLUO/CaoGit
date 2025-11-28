/**
 * Unit tests for the error handler service
 */
import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  createError,
  isAppError,
  parseApiError,
} from '../../src/services/errorHandler';

// Mock the stores
vi.mock('../../src/stores/toastStore', () => ({
  toastStore: {
    error: vi.fn(),
  },
}));

vi.mock('../../src/stores/debugStore', () => ({
  debugStore: {
    addError: vi.fn(),
  },
}));

describe('errorHandler', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('createError()', () => {
    it('should create an error with correct properties', () => {
      const error = createError('REPO_NOT_FOUND', 'Some details');
      expect(error.code).toBe('REPO_NOT_FOUND');
      expect(error.message).toBe('仓库未找到');
      expect(error.details).toBe('Some details');
    });

    it('should handle unknown error codes gracefully', () => {
      const error = createError('UNKNOWN_ERROR');
      expect(error.code).toBe('UNKNOWN_ERROR');
      expect(error.message).toBe('未知错误');
    });
  });

  describe('isAppError()', () => {
    it('should return true for valid AppError objects', () => {
      const error = createError('NETWORK_ERROR');
      expect(isAppError(error)).toBe(true);
    });

    it('should return false for regular Error objects', () => {
      const error = new Error('test');
      expect(isAppError(error)).toBe(false);
    });

    it('should return false for null', () => {
      expect(isAppError(null)).toBe(false);
    });
  });

  describe('parseApiError()', () => {
    it('should return null for successful responses', () => {
      const result = parseApiError({ success: true });
      expect(result).toBeNull();
    });

    it('should detect network errors', () => {
      const result = parseApiError({ success: false, error: '网络连接失败' });
      expect(result?.code).toBe('NETWORK_ERROR');
    });

    it('should detect auth errors', () => {
      const result = parseApiError({ success: false, error: '认证失败' });
      expect(result?.code).toBe('AUTH_FAILED');
    });

    it('should default to GIT_OPERATION_FAILED for unknown errors', () => {
      const result = parseApiError({ success: false, error: 'Some random error' });
      expect(result?.code).toBe('GIT_OPERATION_FAILED');
    });
  });
});
