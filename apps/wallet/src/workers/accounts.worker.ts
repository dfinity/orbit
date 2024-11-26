import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/ic-agent.core';
import { logger } from '~/core/logger.core';
import { AccountBalance, FetchAccountBalancesResult, UUID } from '~/generated/station/station.did';
import { StationService } from '~/services/station.service';
import { ExtractOk } from '~/types/helper.types';
import { arrayBatchMaker, timer, unreachable } from '~/utils/helper.utils';

const DEFAULT_INTERVAL_MS = 10000;
const MAX_BATCH_SIZE = 5;

const accountsToTrack = new Set<UUID>();
let running = false;

export interface AccountsWorker extends Worker {
  postMessage(msg: AccountsWorkerIncomingMessage): void;
  onmessage: ((this: Worker, ev: MessageEvent<AccountsWorkerResponseMessage>) => void) | null;
}

export interface AccountsWorkerStartInput {
  // The canister id to use for the worker to fetch account balances.
  stationId: Principal;
  // The frequency at which the worker should run in milliseconds.
  //
  // Default: 10000 (10 seconds)
  poolIntervalMs?: number;
}

export interface AccountsWorkerTrackInput {
  accountIds: UUID[];
}

export type AccountsWorkerIncomingMessage =
  | { type: 'start'; data: AccountsWorkerStartInput }
  | { type: 'track'; data: AccountsWorkerTrackInput }
  | { type: 'stop' }
  | { type: 'enable' }
  | { type: 'disable' };

export interface AccountsWorkerErrorResponse {
  code: 'ERR_ACCOUNTS_WORKER';
  msg: string;
}

export interface AccountBalancesWorkerResponse {
  balances: Array<[] | [AccountBalance]>;
}

export type AccountsWorkerResponseMessage =
  | { type: 'stopped' }
  | { type: 'error'; data: AccountsWorkerErrorResponse }
  | { type: 'balances'; data: AccountBalancesWorkerResponse };

class AccountsWorkerImpl {
  private timer: NodeJS.Timeout | null = null;
  private enabled: boolean = false;

  constructor(private stationService: StationService = new StationService(icAgent.get())) {}

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
        case 'track':
          accountsToTrack.clear();
          msg.data.accountIds.forEach(id => accountsToTrack.add(id));
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
    accountsToTrack.clear();

    if (this.timer) {
      this.stop();
    }
    this.enabled = true;

    this.stationService.withStationId(data.stationId);
    const poolIntervalMs =
      data.poolIntervalMs && data.poolIntervalMs > 0 ? data.poolIntervalMs : DEFAULT_INTERVAL_MS;

    this.timer = timer(() => this.run(), poolIntervalMs, {
      immediate: true,
    });
  }

  private stop(): void {
    accountsToTrack.clear();

    if (this.timer) {
      clearInterval(this.timer);

      this.timer = null;
    }

    postMessage({ type: 'stopped' } as AccountsWorkerResponseMessage);
  }

  private async run(): Promise<void> {
    if (!this.enabled || running) {
      return;
    }

    try {
      running = true;
      await icAgent.loadIdentity();

      const batchToTrack = arrayBatchMaker(Array.from(accountsToTrack), MAX_BATCH_SIZE);
      const requests = batchToTrack.map(accountIds =>
        this.stationService.fetchAccountBalances({ account_ids: accountIds }).catch(err => {
          logger.error('Failed to update the balance for the given account ids', { err });

          return [] as ExtractOk<FetchAccountBalancesResult>['balances'];
        }),
      );

      const balances = (await Promise.all(requests)).flat();

      postMessage({
        type: 'balances',
        data: {
          balances,
        },
      } as AccountsWorkerResponseMessage);
    } catch (err) {
      logger.error(`Failed to run accounts worker job`, { err });

      postMessage({
        type: 'error',
        data: {
          code: 'ERR_ACCOUNTS_WORKER',
          msg: `Failed to run job: ${err}`,
        },
      } as AccountsWorkerResponseMessage);
    } finally {
      running = false;
    }
  }
}

AccountsWorkerImpl.register();
