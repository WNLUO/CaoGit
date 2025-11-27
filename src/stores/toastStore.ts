import { reactive } from 'vue';

export interface ToastMessage {
  id: number;
  type: 'success' | 'error' | 'info' | 'warning';
  message: string;
  duration: number;
}

let nextId = 1;

export const toastStore = reactive({
  messages: [] as ToastMessage[],

  show(type: ToastMessage['type'], message: string, duration = 3000) {
    const id = nextId++;
    const toast: ToastMessage = {
      id,
      type,
      message,
      duration
    };

    this.messages.push(toast);

    // 自动移除
    if (duration > 0) {
      setTimeout(() => {
        this.remove(id);
      }, duration);
    }

    return id;
  },

  success(message: string, duration = 3000) {
    return this.show('success', message, duration);
  },

  error(message: string, duration = 4000) {
    return this.show('error', message, duration);
  },

  warning(message: string, duration = 3500) {
    return this.show('warning', message, duration);
  },

  info(message: string, duration = 3000) {
    return this.show('info', message, duration);
  },

  remove(id: number) {
    const index = this.messages.findIndex(m => m.id === id);
    if (index !== -1) {
      this.messages.splice(index, 1);
    }
  },

  clear() {
    this.messages = [];
  }
});
