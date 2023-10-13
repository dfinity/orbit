export interface GlobalNotification {
  show: boolean;
  message: string;
  type: 'error' | 'success' | 'info' | 'warning';
}

export interface LoadableItem<T> {
  loading: boolean;
  data: T;
}
