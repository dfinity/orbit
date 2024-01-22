import { unreachable } from '~/core';

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

export class SessionBroadcastChannel {
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
