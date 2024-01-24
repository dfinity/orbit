import { TransferStatus } from '~/generated/wallet/wallet.did';
import { AccountTransferStatus } from '~/types';

export const isSetAndNotFalse = (value: unknown) => {
  if (value === 'false' || value === false || value === undefined || value === null) {
    return false;
  }

  return true;
};

// Formats a balance that is a bigint into a string with the correct number of decimals
export const formatBalance = (amount: bigint, decimals: number): string => {
  const integerPart = amount / BigInt(10 ** decimals);
  const remainder = amount % BigInt(10 ** decimals);
  const integerPartStr = integerPart.toString();
  const remainderStr = remainder.toString().padStart(decimals, '0');

  return `${integerPartStr}.${remainderStr}`;
};

export const startOfDay = (date: Date): Date => {
  const dt = new Date(date.getTime());
  dt.setUTCHours(0, 0, 0, 0);

  return dt;
};

export const endOfDay = (date: Date): Date => {
  const dt = new Date(date.getTime());
  dt.setUTCHours(23, 59, 59, 999);

  return dt;
};

export const amountToBigInt = (amount: string, decimals: number): bigint => {
  const [integer, decimal] = amount.split('.');

  if (decimal?.length > decimals) {
    throw new Error(`Invalid format, amounts can only have ${decimals} decimals`);
  }

  const paddedDecimal = `${decimal ?? ''}`.padEnd(decimals, '0');

  return BigInt(`${integer}${paddedDecimal}`);
};

export const extractTransferStatus = (status: TransferStatus): AccountTransferStatus => {
  if ('Created' in status) {
    return AccountTransferStatus.Created;
  }

  if ('Failed' in status) {
    return AccountTransferStatus.Failed;
  }

  if ('Processing' in status) {
    return AccountTransferStatus.Processing;
  }

  if ('Completed' in status) {
    return AccountTransferStatus.Completed;
  }

  return AccountTransferStatus.Unknown;
};

export const timer = (
  cb: () => void,
  intervalMs = 1000,
  opts: {
    immediate?: boolean;
  } = {
    immediate: true,
  },
): NodeJS.Timeout => {
  if (opts.immediate && intervalMs > 1) {
    cb();
  }

  return setInterval(cb, intervalMs);
};

export const arrayBatchMaker = <T>(array: T[], batchSize: number = 5): T[][] => {
  const batches: T[][] = [];

  array.reduce((acc, item, index) => {
    const groupIndex = Math.floor(index / batchSize);
    if (!acc[groupIndex]) {
      acc[groupIndex] = [];
    }
    acc[groupIndex].push(item);

    return acc;
  }, batches);

  return batches;
};

export function nanoToJsDate(nanoTimestamp: bigint): Date {
  // Convert BigInt to milliseconds by dividing by 1 million
  const milliTimestamp = nanoTimestamp / BigInt(1000000);

  // Convert to number type as JavaScript's Date constructor expects a number for milliseconds
  const jsDate = new Date(Number(milliTimestamp));
  return jsDate;
}

export const wait = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms));
};

export const unreachable = (input: never): never => {
  throw new Error(`Unreachable, found: '${input}'`);
};

export type KeysOfUnion<T extends object> = T extends T ? keyof T : never;

export const variantIs = <EnumType extends object, T extends EnumType>(
  p: EnumType,
  key: KeysOfUnion<T>,
): p is T => {
  return (key as string) in p;
};

export const throttle = <T extends (...args: unknown[]) => unknown>(fn: T, wait: number = 300) => {
  let inThrottle: boolean, lastFn: ReturnType<typeof setTimeout>, lastTime: number;
  return function (this: ThisParameterType<T>, ...args: Parameters<T>) {
    if (!inThrottle) {
      fn.apply(this, args);
      lastTime = Date.now();
      inThrottle = true;
    } else {
      clearTimeout(lastFn);
      lastFn = setTimeout(
        () => {
          if (Date.now() - lastTime >= wait) {
            fn.apply(this, args);
            lastTime = Date.now();
          }
        },
        Math.max(wait - (Date.now() - lastTime), 0),
      );
    }
  };
};
export class ResettableTimeout {
  private timeout: NodeJS.Timeout | null = null;
  private callbacks: (() => void)[] = [];

  constructor() {}

  public subscribe(callback: () => void) {
    this.callbacks.push(callback);
  }

  public unsubscribe(callback: () => void) {
    this.callbacks = this.callbacks.filter(cb => cb !== callback);
  }

  public reset(timeoutMs: number) {
    if (this.timeout !== null) {
      clearTimeout(this.timeout);
    }

    this.timeout = setTimeout(() => {
      for (const callback of this.callbacks) {
        callback();
      }
      this.timeout = null;
    }, timeoutMs);
  }

  public clear() {
    if (this.timeout !== null) {
      clearTimeout(this.timeout);
      this.timeout = null;
    }
  }

  public isActive(): boolean {
    return this.timeout !== null;
  }
}
