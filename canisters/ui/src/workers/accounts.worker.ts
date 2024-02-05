import { Principal } from '@dfinity/principal';
import { logger, timer, unreachable } from '~/core';
import { icAgent } from '~/core/ic-agent';
import { Account } from '~/generated/wallet/wallet.did';
import { WalletService } from '~/services';

const DEFAULT_POOL_INTERVAL_MS = 5000;
const BALANCES_OUTDATED_THRESHOLD_MS = 15000;

export interface AccountsWorker extends Worker {
  postMessage(msg: AccountsWorkerIncomingMessage): void;
  onmessage: ((this: Worker, ev: MessageEvent<AccountsWorkerResponseMessage>) => void) | null;
}

export interface AccountsWorkerStartInput {
  // The wallet id to use for the worker.
  walletId: Principal;
  // The frequency at which the worker should poll for account updates in milliseconds.
  //
  // Default: 30000 (30 seconds)
  poolIntervalMs?: number;
}

export type AccountsWorkerIncomingMessage =
  | {
      type: 'start';
      data: AccountsWorkerStartInput;
    }
  | {
      type: 'stop';
    }
  | {
      type: 'enable';
    }
  | {
      type: 'disable';
    };

export interface AccountsWorkerErrorResponse {
  code: 'ERR_FETCH_ACCOUNTS';
  msg: string;
}

export interface AccountsWorkerResponse {
  accounts: Account[];
}

export type AccountsWorkerResponseMessage =
  | { type: 'stopped' }
  | { type: 'error'; data: AccountsWorkerErrorResponse }
  | { type: 'accounts'; data: AccountsWorkerResponse };

class AccountsWorkerImpl {
  private timer: NodeJS.Timeout | null = null;
  private enabled: boolean = false;

  constructor(private walletService: WalletService = new WalletService()) {}

  static register(): void {
    if (typeof navigator === 'undefined') {
      throw new Error('Worker can only be registered in the browser');
    }

    const worker = new AccountsWorkerImpl();

    onmessage = ({ data: msg }: MessageEvent<AccountsWorkerIncomingMessage>) => {
      switch (msg.type) {
        case 'start':
          worker.start(msg.data);
          break;
        case 'stop':
          worker.stop();
          break;
        case 'enable':
          worker.enabled = true;
          break;
        case 'disable':
          worker.enabled = false;
          break;
        default:
          unreachable(msg);
      }
    };
  }

  private start(data: AccountsWorkerStartInput): void {
    if (this.timer) {
      this.stop();
    }
    this.enabled = true;

    this.walletService.withWalletId(data.walletId);
    const poolIntervalMs =
      data.poolIntervalMs && data.poolIntervalMs > 0
        ? data.poolIntervalMs
        : DEFAULT_POOL_INTERVAL_MS;

    this.timer = timer(() => this.refreshAccounts(), poolIntervalMs, {
      immediate: true,
    });
  }

  private stop(): void {
    if (this.timer) {
      clearInterval(this.timer);

      this.timer = null;
    }

    postMessage({ type: 'stopped' } as AccountsWorkerResponseMessage);
  }

  private async refreshAccounts(): Promise<void> {
    if (!this.enabled) {
      return;
    }

    try {
      await icAgent.loadIdentity();

      const result = await this.walletService.listAccounts();
      const accounts = result.accounts;

      const balancesOutdatedAccounts = accounts.filter(account => {
        if (!account.balance?.[0]) {
          return true;
        }

        const lastUpdated = new Date(account.balance[0].last_update_timestamp);
        const now = new Date();
        const diff = now.getTime() - lastUpdated.getTime();

        return diff > BALANCES_OUTDATED_THRESHOLD_MS;
      });

      // This will update the account balances in the background, the balances will be updated
      // in the next polling cycle if already available.
      this.refreshAccountBalances(balancesOutdatedAccounts);

      postMessage({
        type: 'accounts',
        data: {
          accounts,
        },
      } as AccountsWorkerResponseMessage);
    } catch (err) {
      logger.error(`Failed to fetch accounts`, { err });

      postMessage({
        type: 'error',
        data: {
          code: 'ERR_FETCH_ACCOUNTS',
          msg: `Failed to fetch accounts: ${err}`,
        },
      } as AccountsWorkerResponseMessage);
    }
  }

  private async refreshAccountBalances(accounts: Account[]): Promise<void> {
    if (accounts.length === 0) {
      return;
    }

    await this.walletService
      .fetchAccountBalances({
        account_ids: accounts.map(account => account.id),
      })
      .catch(err => {
        logger.error('Failed to update the balance for the given account ids', { err });
      });
  }
}

AccountsWorkerImpl.register();
