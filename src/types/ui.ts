/**
 * UI-related types
 */

export interface ToastMessage {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
}

export interface ContextMenuItem {
  label: string;
  icon?: string;
  action: () => void;
  disabled?: boolean;
  separator?: boolean;
}

export interface FilterOptions {
  searchText: string;
  author: string;
  dateFrom: string;
  dateTo: string;
  branch: string;
}
