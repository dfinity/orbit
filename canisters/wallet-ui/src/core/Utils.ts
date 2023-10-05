import { TransferStatus } from '~/generated/bank/bank.did';
import { WalletTransferStatus } from '~/types';

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

export const amountToBigInt = (amount: string, decimals: number): bigint => {
  const [integer, decimal] = amount.split('.');

  if (decimal?.length > decimals) {
    throw new Error(`Invalid format, amounts can only have ${decimals} decimals`);
  }

  const paddedDecimal = `${decimal ?? ''}`.padEnd(decimals, '0');

  return BigInt(`${integer}${paddedDecimal}`);
};

export const extractTransferStatus = (status: TransferStatus): WalletTransferStatus => {
  if ('Pending' in status) {
    return WalletTransferStatus.Pending;
  }

  if ('Approved' in status) {
    return WalletTransferStatus.Approved;
  }

  if ('Rejected' in status) {
    return WalletTransferStatus.Rejected;
  }

  if ('Cancelled' in status) {
    return WalletTransferStatus.Cancelled;
  }

  if ('Submitted' in status) {
    return WalletTransferStatus.Submitted;
  }

  if ('Processing' in status) {
    return WalletTransferStatus.Processing;
  }

  if ('Completed' in status) {
    return WalletTransferStatus.Completed;
  }

  return WalletTransferStatus.Unknown;
};
