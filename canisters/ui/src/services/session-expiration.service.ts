import { ResettableTimeout, unreachable } from '~/utils/helper.utils';

export type SessionExpirationMessage =
  | 'otherTabSignedIn'
  | 'otherTabSignedOut'
  | 'otherTabActive'
  | 'userInactive'
  | 'sessionExpired';

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

export class SessionExpirationService {
  static readonly INACTIVITY_TIMEOUT_MS = 1000 * 60 * 10; // 10 minutes

  private callbacks: ((msg: SessionExpirationMessage) => void)[] = [];

  constructor(
    private channel: BroadcastChannel = new BroadcastChannel('session'),
    private sessionTimeout = new ResettableTimeout(),
    private inactivityTimeout = new ResettableTimeout(),
  ) {
    this.channel.onmessage = msg => this.onChannelMessage(msg.data);
    this.sessionTimeout.subscribe(() => this.onSessionTimeout());
    this.inactivityTimeout.subscribe(() => this.onInactivityTimeout());
  }

  public subscribe(callback: (msg: SessionExpirationMessage) => void) {
    this.callbacks.push(callback);
  }

  public unsubscribe(callback: (msg: SessionExpirationMessage) => void) {
    this.callbacks = this.callbacks.filter(cb => cb !== callback);
  }

  public resetSessionTimeout(timeoutMs: number) {
    this.sessionTimeout.reset(timeoutMs);
  }

  public resetInactivityTimeout() {
    this.inactivityTimeout.reset(SessionExpirationService.INACTIVITY_TIMEOUT_MS);
  }

  public isUserActive() {
    return this.inactivityTimeout.isActive();
  }

  public clearSessionTimer() {
    this.sessionTimeout.clear();
  }

  public clearInactivityTimer() {
    this.inactivityTimeout.clear();
  }

  public notifyActive() {
    this.channel.postMessage({ type: 'active' });
  }

  public notifySignedIn() {
    this.channel.postMessage({ type: 'signin' });
  }

  public notifySignedOut() {
    this.channel.postMessage({ type: 'signout' });
  }

  public registerActivity() {
    if (this.isUserActive()) {
      this.notifyActive();
      this.resetInactivityTimeout();
    }
  }

  private notify(msg: SessionExpirationMessage) {
    for (const callback of this.callbacks) {
      callback(msg);
    }
  }

  private onChannelMessage(msg: BroadcastChannelMessage) {
    switch (msg.type) {
      case 'active':
        this.notify('otherTabActive');
        break;
      case 'signout':
        this.notify('otherTabSignedOut');
        break;
      case 'signin':
        this.notify('otherTabSignedIn');
        break;
      default:
        unreachable(msg);
    }
  }

  private onSessionTimeout() {
    this.notify('sessionExpired');
  }

  private onInactivityTimeout() {
    this.notify('userInactive');
  }
}
