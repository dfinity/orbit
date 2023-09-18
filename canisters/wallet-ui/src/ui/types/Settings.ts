export interface GlobalNotification {
  show: boolean;
  message: string;
  type: 'error' | 'success' | 'info' | 'warning';
}
