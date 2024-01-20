import { logger, unreachable } from '~/core';
import { services } from '.';

type BroadcastChannelMessage =
  | {
      type: 'signout';
    }
  | {
      type: 'signin';
    }
  | {
      type: 'active';
    };

export abstract class BroadcastChannellike<T> {
  constructor(protected onMessage: (msg: T) => void) {}
  abstract postMessage(msg: T): void;
}

export class BrowserBroadcastChannel<T> extends BroadcastChannellike<T> {
  private channel: BroadcastChannel = new BroadcastChannel('session');
  constructor(onMessage: (msg: T) => void) {
    super(onMessage);
    this.channel.onmessage = msg => onMessage(msg.data);
  }

  postMessage(msg: T): void {
    this.channel.postMessage(msg);
  }
}

export class AuthCheck {
  inactiveTimeout: NodeJS.Timeout | null = null;
  sessionExpirationTimeout: NodeJS.Timeout | null = null;

  constructor(
    private config: {
      inactivityTimeoutMs: number;
      onExpired?: () => void;
      onInactive?: () => void;
      onOtherTabSignout?: () => void;
      onOtherTabReauthenticate?: () => void;
    },
    private broadcastChannel: BroadcastChannellike<BroadcastChannelMessage> = new BrowserBroadcastChannel(
      msg => {
        this.onMessage(msg);
      },
    ),
  ) {
    this.resetInactiveTimeout();
  }

  public setActive() {
    this.resetInactiveTimeout();
  }

  public notifyActive() {
    this.broadcastChannel.postMessage({ type: 'active' });
  }

  public setSignedIn() {
    this.setSessionExpiration();
    this.resetInactiveTimeout();
  }

  public notifySignedIn() {
    this.broadcastChannel.postMessage({ type: 'signin' });
  }

  public setSignedOut() {
    if (this.sessionExpirationTimeout !== null) {
      clearTimeout(this.sessionExpirationTimeout);
      this.sessionExpirationTimeout = null;
    }

    if (this.inactiveTimeout !== null) {
      clearTimeout(this.inactiveTimeout);
      this.inactiveTimeout = null;
    }
  }
  public notifySignedOut() {
    this.broadcastChannel.postMessage({ type: 'signout' });
  }

  private async setSessionExpiration() {
    if (this.sessionExpirationTimeout !== null) {
      clearTimeout(this.sessionExpirationTimeout);
    }

    const authService = services().auth;
    const maybeRemainingSessionTimeMs = await authService.getRemainingSessionTimeMs();
    if (maybeRemainingSessionTimeMs !== null) {
      this.sessionExpirationTimeout = setTimeout(() => {
        this.config.onExpired?.();
      }, maybeRemainingSessionTimeMs);
    }
  }

  private resetInactiveTimeout() {
    if (this.inactiveTimeout !== null) {
      clearTimeout(this.inactiveTimeout);
    }

    this.inactiveTimeout = setTimeout(() => {
      this.config.onInactive?.();
    }, this.config.inactivityTimeoutMs);
  }

  private onMessage(msg: BroadcastChannelMessage) {
    logger.info(`[BROADCASE] onMessage:`, msg);

    switch (msg.type) {
      case 'active':
        this.resetInactiveTimeout();
        break;
      case 'signout':
        this.config.onOtherTabSignout?.();
        this.setSignedOut();
        break;
      case 'signin':
        this.config.onOtherTabReauthenticate?.();
        this.setSessionExpiration();
        break;
      default:
        unreachable(msg);
    }
  }
}
