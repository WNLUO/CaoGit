import { reactive } from 'vue';

export interface NetworkMetrics {
  // 当前传输状态
  status: 'idle' | 'uploading' | 'downloading';

  // 最新的速度数据
  downloadSpeed: number; // KB/s
  uploadSpeed: number; // KB/s
  latency: number; // ms

  // 传输进度信息
  currentSize: number; // 已传输大小 (bytes)
  totalSize: number; // 总大小 (bytes)
  operation: string; // 当前操作名称，如 "push", "pull", "fetch"

  // 时间戳
  lastUpdated: number; // 最后更新时间
}

const DEFAULT_METRICS: NetworkMetrics = {
  status: 'idle',
  downloadSpeed: 0,
  uploadSpeed: 0,
  latency: 0,
  currentSize: 0,
  totalSize: 0,
  operation: '',
  lastUpdated: 0
};

export const networkMetricsStore = reactive({
  metrics: { ...DEFAULT_METRICS },

  // 更新下载速度（KB/s）
  setDownloadSpeed(speed: number) {
    this.metrics.downloadSpeed = Math.round(speed);
    this.metrics.lastUpdated = Date.now();
  },

  // 更新上传速度（KB/s）
  setUploadSpeed(speed: number) {
    this.metrics.uploadSpeed = Math.round(speed);
    this.metrics.lastUpdated = Date.now();
  },

  // 更新延迟（ms）
  setLatency(latency: number) {
    this.metrics.latency = Math.round(latency);
    this.metrics.lastUpdated = Date.now();
  },

  // 设置传输状态
  setStatus(status: 'idle' | 'uploading' | 'downloading') {
    this.metrics.status = status;
    this.metrics.lastUpdated = Date.now();
  },

  // 设置传输进度
  setProgress(currentSize: number, totalSize: number) {
    this.metrics.currentSize = currentSize;
    this.metrics.totalSize = totalSize;
    this.metrics.lastUpdated = Date.now();
  },

  // 设置当前操作
  setOperation(operation: string) {
    this.metrics.operation = operation;
    this.metrics.lastUpdated = Date.now();
  },

  // 开始上传操作
  startUpload(operation: string, totalSize: number) {
    this.metrics.status = 'uploading';
    this.metrics.operation = operation;
    this.metrics.totalSize = totalSize;
    this.metrics.currentSize = 0;
    this.metrics.uploadSpeed = 0;
    this.metrics.lastUpdated = Date.now();
  },

  // 开始下载操作
  startDownload(operation: string, totalSize: number = 0) {
    this.metrics.status = 'downloading';
    this.metrics.operation = operation;
    this.metrics.totalSize = totalSize;
    this.metrics.currentSize = 0;
    this.metrics.downloadSpeed = 0;
    this.metrics.lastUpdated = Date.now();
  },

  // 完成传输
  finish() {
    this.metrics.status = 'idle';
    this.metrics.operation = '';
    this.metrics.currentSize = 0;
    this.metrics.totalSize = 0;
    this.metrics.lastUpdated = Date.now();
  },

  // 重置所有数据
  reset() {
    this.metrics = { ...DEFAULT_METRICS };
  }
});
