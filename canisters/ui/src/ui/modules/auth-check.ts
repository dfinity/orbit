import { logger, unreachable } from '~/core';

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

export class SessionBroadcaseChannel {
  private channel: BroadcastChannel;
  constructor(
    private config: {
      channel?: BroadcastChannel;
      onOtherTabActive?: () => void;
      onOtherTabSignout?: () => void;
      onOtherTabSignin?: () => void;
    },
  ) {
    this.channel = config.channel || new BroadcastChannel('session');
    this.channel.onmessage = msg => this.onMessage(msg.data);
  }

  private onMessage(msg: BroadcastChannelMessage) {
    logger.info(`[BROADCASE] onMessage:`, msg);

    switch (msg.type) {
      case 'active':
        this.config.onOtherTabActive?.();
        break;
      case 'signout':
        this.config.onOtherTabSignout?.();
        break;
      case 'signin':
        this.config.onOtherTabSignin?.();
        break;
      default:
        unreachable(msg);
    }
  }

  private postMessage(msg: BroadcastChannelMessage): void {
    this.channel.postMessage(msg);
  }

  public notifyActive() {
    this.postMessage({ type: 'active' });
  }

  public notifySignedIn() {
    this.postMessage({ type: 'signin' });
  }

  public notifySignedOut() {
    this.postMessage({ type: 'signout' });
  }
}

export class Timeout {
  private timeout: NodeJS.Timeout | null = null;
  constructor(private callback: () => void) {}

  public reset(timeoutMs: number) {
    if (this.timeout !== null) {
      clearTimeout(this.timeout);
    }

    this.timeout = setTimeout(() => {
      this.callback();
    }, timeoutMs);
  }

  public clear() {
    if (this.timeout !== null) {
      clearTimeout(this.timeout);
      this.timeout = null;
    }
  }
}

export class FixedTimeout extends Timeout {
  constructor(
    callback: () => void,
    private timeoutMs: number,
  ) {
    super(callback);
    this.reset();
  }

  public reset() {
    super.reset(this.timeoutMs);
  }
}
