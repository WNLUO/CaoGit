import { reactive, ref } from 'vue';

export interface ErrorLog {
  id: number;
  timestamp: string;
  message: string;
  stack?: string;
  context?: string;
  type: 'error' | 'warning' | 'info';
}

function loadDebugMode(): boolean {
  try {
    const saved = localStorage.getItem('debug_mode');
    return saved === 'true';
  } catch {
    return false;
  }
}

const showErrorDialogRef = ref(false);

export const debugStore = reactive({
  enabled: loadDebugMode(),
  errors: [] as ErrorLog[],
  get showErrorDialog() {
    return showErrorDialogRef;
  },
  currentError: null as ErrorLog | null,

  setDebugMode(enabled: boolean) {
    this.enabled = enabled;
    try {
      localStorage.setItem('debug_mode', String(enabled));
    } catch (error) {
      console.error('Failed to save debug mode:', error);
    }
  },

  logError(message: string, error?: Error, context?: string) {
    const errorLog: ErrorLog = {
      id: Date.now(),
      timestamp: new Date().toISOString(),
      message,
      stack: error?.stack,
      context,
      type: 'error'
    };

    this.errors.push(errorLog);

    // Keep only last 100 errors
    if (this.errors.length > 100) {
      this.errors.shift();
    }

    // Console log for development
    console.error(`[Debug] ${context || 'Error'}:`, message, error);

    // Show dialog if debug mode is enabled
    if (this.enabled) {
      this.currentError = errorLog;
      showErrorDialogRef.value = true;
    }
  },

  logWarning(message: string, context?: string) {
    const errorLog: ErrorLog = {
      id: Date.now(),
      timestamp: new Date().toISOString(),
      message,
      context,
      type: 'warning'
    };

    this.errors.push(errorLog);

    if (this.errors.length > 100) {
      this.errors.shift();
    }

    console.warn(`[Debug] ${context || 'Warning'}:`, message);

    if (this.enabled) {
      this.currentError = errorLog;
      showErrorDialogRef.value = true;
    }
  },

  logInfo(message: string, context?: string) {
    const errorLog: ErrorLog = {
      id: Date.now(),
      timestamp: new Date().toISOString(),
      message,
      context,
      type: 'info'
    };

    this.errors.push(errorLog);

    if (this.errors.length > 100) {
      this.errors.shift();
    }

    console.info(`[Debug] ${context || 'Info'}:`, message);
  },

  getFormattedError(error: ErrorLog): string {
    const lines = [
      `时间: ${new Date(error.timestamp).toLocaleString('zh-CN')}`,
      `类型: ${error.type}`,
      `上下文: ${error.context || 'N/A'}`,
      `消息: ${error.message}`,
    ];

    if (error.stack) {
      lines.push(`\n堆栈:\n${error.stack}`);
    }

    return lines.join('\n');
  },

  copyErrorToClipboard(error: ErrorLog) {
    const text = this.getFormattedError(error);

    if (navigator.clipboard) {
      navigator.clipboard.writeText(text)
        .then(() => {
          alert('错误信息已复制到剪贴板');
        })
        .catch((err) => {
          console.error('Failed to copy:', err);
          // Fallback: show text in a prompt
          prompt('复制以下内容:', text);
        });
    } else {
      // Fallback for older browsers
      prompt('复制以下内容:', text);
    }
  },

  clearErrors() {
    this.errors = [];
  },

  closeErrorDialog() {
    showErrorDialogRef.value = false;
    this.currentError = null;
  }
});

// Global error handler
if (typeof window !== 'undefined') {
  window.addEventListener('error', (event) => {
    debugStore.logError(
      event.message,
      event.error,
      `Global Error Handler (${event.filename}:${event.lineno}:${event.colno})`
    );
  });

  window.addEventListener('unhandledrejection', (event) => {
    debugStore.logError(
      `Unhandled Promise Rejection: ${event.reason}`,
      event.reason instanceof Error ? event.reason : undefined,
      'Promise Rejection'
    );
  });
}
