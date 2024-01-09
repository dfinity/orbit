import { AnonymousIdentity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { logger, timer } from '~/core';
import { icAgent } from '~/core/ic-agent';
import { Notification } from '~/generated/wallet/wallet.did';
import { WalletService } from '~/services';
import { loadIdentity } from '~/workers/utils';

const DEFAULT_POOL_INTERVAL_MS = 5000;

export interface NotificationsWorker extends Worker {
  postMessage(msg: NotificationsWorkerIncomingMessage): void;
  onmessage: ((this: Worker, ev: MessageEvent<NotificationsWorkerResponseMessage>) => void) | null;
}

export interface NotificationsWorkerStartInput {
  // The wallet id to use for the worker.
  walletId: Principal;
  // The frequency at which the worker should poll for notification updates in milliseconds.
  //
  // Default: 30000 (30 seconds)
  poolIntervalMs?: number;
}

export type NotificationsWorkerIncomingMessage =
  | {
      type: 'start';
      data: NotificationsWorkerStartInput;
    }
  | {
      type: 'stop';
    };

export interface NotificationsWorkerErrorResponse {
  code: 'ERR_FETCH_NOTIFICATIONS';
  msg: string;
}

export interface NotificationsWorkerResponse {
  notifications: Notification[];
}

export type NotificationsWorkerResponseMessage =
  | { type: 'stopped' }
  | { type: 'error'; data: NotificationsWorkerErrorResponse }
  | { type: 'notifications'; data: NotificationsWorkerResponse };

class NotificationsWorkerImpl {
  private timer: NodeJS.Timeout | null = null;
  private lastNotificationId: string | null = null;
  private lastNotificationDate: Date | null = null;

  constructor(private walletService: WalletService = new WalletService()) {}

  static register(): void {
    if (typeof navigator === 'undefined') {
      throw new Error('Worker can only be registered in the browser');
    }

    const worker = new NotificationsWorkerImpl();

    onmessage = ({ data: msg }: MessageEvent<NotificationsWorkerIncomingMessage>) => {
      switch (msg.type) {
        case 'start':
          worker.start(msg.data);
          break;
        case 'stop':
          worker.stop();
          break;
      }
    };
  }

  private start(data: NotificationsWorkerStartInput): void {
    if (this.timer) {
      this.stop();
    }

    this.lastNotificationDate = null;
    this.lastNotificationId = null;

    this.walletService.withWalletId(data.walletId);
    const poolIntervalMs =
      data.poolIntervalMs && data.poolIntervalMs > 0
        ? data.poolIntervalMs
        : DEFAULT_POOL_INTERVAL_MS;

    this.timer = timer(() => this.refreshNotifications(), poolIntervalMs, {
      immediate: true,
    });
  }

  private stop(): void {
    if (this.timer) {
      clearInterval(this.timer);

      this.timer = null;
    }

    postMessage({ type: 'stopped' } as NotificationsWorkerResponseMessage);
  }

  private async refreshNotifications(): Promise<void> {
    try {
      const identity = (await loadIdentity()) ?? new AnonymousIdentity();
      await icAgent.init();
      icAgent.get().replaceIdentity(identity);
      if (identity.getPrincipal().isAnonymous()) {
        logger.warn('Using anonymous identity, make sure to sign in to the wallet');
      }

      const notifications = await this.walletService.listUnreadNotifications(
        this.lastNotificationDate ?? undefined,
        this.lastNotificationId ?? undefined,
      );

      this.lastNotificationId = notifications[0]?.id ?? null;
      this.lastNotificationDate = notifications[0]
        ? new Date(notifications[0].created_at)
        : new Date();

      postMessage({
        type: 'notifications',
        data: {
          notifications,
        },
      } as NotificationsWorkerResponseMessage);
    } catch (err) {
      logger.error(`Failed to fetch notifications`, { err });

      postMessage({
        type: 'error',
        data: {
          code: 'ERR_FETCH_NOTIFICATIONS',
          msg: `Failed to fetch notifications: ${err}`,
        },
      } as NotificationsWorkerResponseMessage);
    }
  }
}

NotificationsWorkerImpl.register();
