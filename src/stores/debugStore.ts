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
  },

  // 日志过滤
  getFilteredErrors(type?: 'error' | 'warning' | 'info', searchText?: string) {
    let filtered = this.errors;

    if (type) {
      filtered = filtered.filter(e => e.type === type);
    }

    if (searchText) {
      const lowerSearch = searchText.toLowerCase();
      filtered = filtered.filter(e =>
        e.message.toLowerCase().includes(lowerSearch) ||
        e.context?.toLowerCase().includes(lowerSearch)
      );
    }

    return filtered;
  },

  // 获取错误统计
  getErrorStats() {
    return {
      total: this.errors.length,
      errors: this.errors.filter(e => e.type === 'error').length,
      warnings: this.errors.filter(e => e.type === 'warning').length,
      infos: this.errors.filter(e => e.type === 'info').length,
    };
  },

  // 导出日志为 JSON
  exportLogsAsJson() {
    const data = {
      exportTime: new Date().toISOString(),
      stats: this.getErrorStats(),
      logs: this.errors.map(e => this.getFormattedError(e))
    };
    const json = JSON.stringify(data, null, 2);
    this.downloadFile(json, `logs_${Date.now()}.json`, 'application/json');
  },

  // 导出日志为 CSV
  exportLogsAsCSV() {
    const headers = ['时间', '类型', '消息', '上下文'];
    const rows = this.errors.map(e => [
      new Date(e.timestamp).toLocaleString('zh-CN'),
      e.type,
      `"${e.message.replace(/"/g, '""')}"`,
      `"${(e.context || '').replace(/"/g, '""')}"`
    ]);

    const csv = [headers, ...rows]
      .map(row => row.join(','))
      .join('\n');

    this.downloadFile(csv, `logs_${Date.now()}.csv`, 'text/csv');
  },

  // 导出日志为文本
  exportLogsAsText() {
    const lines = [
      `=== Git管理器 调试日志 ===`,
      `导出时间: ${new Date().toLocaleString('zh-CN')}`,
      ``,
      `统计信息:`,
      `  总数: ${this.errors.length}`,
      `  错误: ${this.errors.filter(e => e.type === 'error').length}`,
      `  警告: ${this.errors.filter(e => e.type === 'warning').length}`,
      `  信息: ${this.errors.filter(e => e.type === 'info').length}`,
      ``,
      `═════════════════════════════════════`,
      ...this.errors.map(e => this.getFormattedError(e)).map(e => `\n${e}\n`)
    ];

    this.downloadFile(lines.join('\n'), `logs_${Date.now()}.txt`, 'text/plain');
  },

  // 辅助方法：下载文件
  downloadFile(content: string, filename: string, mimeType: string) {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  },

  // 按时间范围清除日志
  clearErrorsByDateRange(startDate: Date, endDate: Date) {
    const start = startDate.getTime();
    const end = endDate.getTime();
    this.errors = this.errors.filter(e => {
      const time = new Date(e.timestamp).getTime();
      return time < start || time > end;
    });
  },

  // 按类型清除日志
  clearErrorsByType(type: 'error' | 'warning' | 'info') {
    this.errors = this.errors.filter(e => e.type !== type);
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
