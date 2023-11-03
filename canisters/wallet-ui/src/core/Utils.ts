import { TransferStatus } from '~/generated/bank/bank.did';
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
  if ('Pending' in status) {
    return AccountTransferStatus.Pending;
  }

  if ('Approved' in status) {
    return AccountTransferStatus.Approved;
  }

  if ('Rejected' in status) {
    return AccountTransferStatus.Rejected;
  }

  if ('Failed' in status) {
    return AccountTransferStatus.Failed;
  }

  if ('Cancelled' in status) {
    return AccountTransferStatus.Cancelled;
  }

  if ('Submitted' in status) {
    return AccountTransferStatus.Submitted;
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
  if (opts.immediate) {
    setTimeout(() => cb(), 100);
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
