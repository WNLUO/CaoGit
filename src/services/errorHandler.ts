/**
 * Centralized Error Handling Service
 *
 * Provides consistent error handling across the application.
 */

import { toastStore } from '../stores/toastStore';
import { debugStore } from '../stores/debugStore';

export interface AppError {
  code: string;
  message: string;
  details?: string;
  originalError?: unknown;
}

export type ErrorCode =
  | 'REPO_NOT_FOUND'
  | 'REPO_OPEN_FAILED'
  | 'GIT_OPERATION_FAILED'
  | 'NETWORK_ERROR'
  | 'AUTH_FAILED'
  | 'CONFLICT_DETECTED'
  | 'INVALID_INPUT'
  | 'UNKNOWN_ERROR';

const ERROR_MESSAGES: Record<ErrorCode, { zh: string; en: string }> = {
  REPO_NOT_FOUND: {
    zh: '仓库未找到',
    en: 'Repository not found',
  },
  REPO_OPEN_FAILED: {
    zh: '无法打开仓库',
    en: 'Failed to open repository',
  },
  GIT_OPERATION_FAILED: {
    zh: 'Git 操作失败',
    en: 'Git operation failed',
  },
  NETWORK_ERROR: {
    zh: '网络连接失败',
    en: 'Network connection failed',
  },
  AUTH_FAILED: {
    zh: '认证失败',
    en: 'Authentication failed',
  },
  CONFLICT_DETECTED: {
    zh: '检测到冲突',
    en: 'Conflict detected',
  },
  INVALID_INPUT: {
    zh: '输入无效',
    en: 'Invalid input',
  },
  UNKNOWN_ERROR: {
    zh: '未知错误',
    en: 'Unknown error',
  },
};

/**
 * Create a standardized application error
 */
export function createError(
  code: ErrorCode,
  details?: string,
  originalError?: unknown
): AppError {
  const messages = ERROR_MESSAGES[code] || ERROR_MESSAGES.UNKNOWN_ERROR;
  return {
    code,
    message: messages.zh, // Default to Chinese
    details,
    originalError,
  };
}

/**
 * Handle an error by logging it and showing a toast notification
 */
export function handleError(error: unknown, context?: string): void {
  let appError: AppError;

  if (isAppError(error)) {
    appError = error;
  } else if (error instanceof Error) {
    appError = createError('UNKNOWN_ERROR', error.message, error);
  } else if (typeof error === 'string') {
    appError = createError('UNKNOWN_ERROR', error);
  } else {
    appError = createError('UNKNOWN_ERROR', String(error));
  }

  // Log to debug store
  debugStore.logError(appError.message, appError.originalError as Error, context);

  // Show toast notification
  toastStore.error(appError.details || appError.message);

  // Log to console in development
  if (import.meta.env.DEV) {
    console.error(`[${context || 'App'}]`, appError);
  }
}

/**
 * Type guard for AppError
 */
export function isAppError(error: unknown): error is AppError {
  return (
    typeof error === 'object' &&
    error !== null &&
    'code' in error &&
    'message' in error
  );
}

/**
 * Wrap an async function with error handling
 */
export function withErrorHandling<T extends (...args: unknown[]) => Promise<unknown>>(
  fn: T,
  context?: string
): T {
  return (async (...args: Parameters<T>) => {
    try {
      return await fn(...args);
    } catch (error) {
      handleError(error, context);
      throw error;
    }
  }) as T;
}

/**
 * Parse backend API error response
 */
export function parseApiError(response: { success: boolean; error?: string }): AppError | null {
  if (response.success) {
    return null;
  }

  const errorMessage = response.error || 'Unknown error';

  // Detect error type from message
  if (errorMessage.includes('not found') || errorMessage.includes('未找到')) {
    return createError('REPO_NOT_FOUND', errorMessage);
  }
  if (errorMessage.includes('network') || errorMessage.includes('网络')) {
    return createError('NETWORK_ERROR', errorMessage);
  }
  if (errorMessage.includes('auth') || errorMessage.includes('认证')) {
    return createError('AUTH_FAILED', errorMessage);
  }
  if (errorMessage.includes('conflict') || errorMessage.includes('冲突')) {
    return createError('CONFLICT_DETECTED', errorMessage);
  }

  return createError('GIT_OPERATION_FAILED', errorMessage);
}
