import { logger, timer, unreachable } from '~/core';
import { DelegationChain, isDelegationValid } from '@dfinity/identity';
import { IdbStorage, KEY_STORAGE_DELEGATION } from '@dfinity/auth-client';
export const POLL_INTERVAL_MS = 1000;

export interface AuthWorker extends Worker {
  postMessage(msg: AuthWorkerIncomingMessage): void;
  onmessage: ((this: Worker, ev: MessageEvent<AuthWorkerResponseMessage>) => void) | null;
}

export type AuthWorkerIncomingMessage =
  | {
      type: 'start';
    }
  | {
      type: 'stop';
    };

export type AuthWorkerResponseMessage =
  | { type: 'sessionExpired' }
  | { type: 'sessionValid' }
  | { type: 'signedOut' };

export interface DelegationChecker {
  getDelegationState(): Promise<'missing' | 'expired' | 'valid'>;
}

class DelegationCheckerImpl implements DelegationChecker {
  public async getDelegationState(): Promise<'missing' | 'expired' | 'valid'> {
    logger.info('Checking delegation validity');

    const idbStorage: IdbStorage = new IdbStorage();
    const delegationChain: string | null = await idbStorage.get(KEY_STORAGE_DELEGATION);

    const delegation = delegationChain !== null ? DelegationChain.fromJSON(delegationChain) : null;

    if (delegation !== null) {
      if (isDelegationValid(delegation)) {
        return 'valid';
      } else {
        return 'expired';
      }
    }

    return 'missing';
  }
}

export class AuthWorkerImpl {
  private timer: NodeJS.Timeout | null = null;
  private lastDelegationState: Awaited<ReturnType<DelegationCheckerImpl['getDelegationState']>> =
    'valid';

  constructor(private delegationChecker: DelegationChecker = new DelegationCheckerImpl()) {}

  static register(): void {
    if (typeof navigator === 'undefined') {
      throw new Error('Worker can only be registered in the browser');
    }

    const worker = new AuthWorkerImpl();

    onmessage = ({ data: msg }: MessageEvent<AuthWorkerIncomingMessage>) => {
      switch (msg.type) {
        case 'start':
          worker.start();
          break;
        case 'stop':
          worker.stop();
          break;
        default:
          unreachable(msg);
      }
    };
  }

  public start(): void {
    logger.info('Starting auth poll');
    if (this.timer) {
      this.stop();
    }

    this.timer = timer(() => this.doAuthenticationCheck(), POLL_INTERVAL_MS, {
      immediate: false,
    });
  }

  public stop(): void {
    logger.info('Stopping auth poll');
    if (this.timer) {
      clearInterval(this.timer);

      this.timer = null;
    }
  }

  private async doAuthenticationCheck() {
    const delegationState = await this.delegationChecker.getDelegationState();

    if (this.lastDelegationState === delegationState) {
      return;
    }

    this.lastDelegationState = delegationState;

    switch (delegationState) {
      case 'missing':
        logger.info('Signout detected');
        postMessage({ type: 'signedOut' });
        break;
      case 'expired':
        logger.info('Session expired');
        postMessage({ type: 'sessionExpired' });
        break;
      case 'valid':
        logger.info('Session valid');
        postMessage({ type: 'sessionValid' });
        break;
      default:
        unreachable(delegationState);
    }

    if (!delegationState) {
    } else {
    }
  }
}

AuthWorkerImpl.register();
