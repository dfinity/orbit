import { TransferStatus } from '~/generated/station/station.did';
import { AccountTransferStatus } from '~/types/station.types';
import type { IDL as CandidIDL } from '@dfinity/candid';

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

export const isSetAndNotFalse = (value: unknown) => {
  if (value === 'false' || value === false || value === undefined || value === null) {
    return false;
  }

  return true;
};

export const isValidSha256 = (value: string): boolean => {
  return /^[a-f0-9]{64}$/i.test(value);
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

export const assertAndReturn = <T>(value: T | undefined | null, name = 'Value'): T => {
  if (value === undefined || value === null) {
    throw new Error(`${name} is undefined or null.`);
  }

  return value;
};

export const isValidUUID = (uuid: string): boolean => {
  return /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(uuid);
};

export const stringify = (obj: unknown): string => {
  return JSON.stringify(
    obj,
    (_, value) => (typeof value === 'bigint' ? value.toString() : value), // return everything else unchanged
  );
};

// This function is used to transform the IDL to only have update calls, all query and composite query calls are
// converted to update calls. This is useful to be able to dynamically leverage verification of calls for security
// critical contexts.
export const transformIdlWithOnlyVerifiedCalls = (
  build: CandidIDL.InterfaceFactory,
): ((idl: Parameters<CandidIDL.InterfaceFactory>[0]) => CandidIDL.ServiceClass) => {
  const queryAnnotationTypes = ['query', 'composite_query'];

  return (idl): CandidIDL.ServiceClass => {
    const service = build(idl);

    for (const key in service._fields) {
      const annotations: string[] = service._fields[key]?.[1].annotations ?? [];

      if (queryAnnotationTypes.some(type => annotations.includes(type))) {
        service._fields[key][1].annotations = annotations.filter(
          annotation => !queryAnnotationTypes.includes(annotation),
        );
      }
    }

    return service;
  };
};

// Checks if a string is with the correct format for a semantic version.
//
// More information on semantic versioning can be found at: https://semver.org/
export const isSemanticVersion = (version: string, prefix = ''): boolean => {
  let versionWithoutPrefix = version;
  if (version.startsWith(prefix)) {
    versionWithoutPrefix = version.slice(prefix.length);
  }

  return /^((([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)$/.test(
    versionWithoutPrefix,
  );
};

export const removeBasePathFromPathname = (pathname: string, basePath: string): string => {
  const updatedPath = pathname.startsWith(basePath) ? pathname.slice(basePath.length) : pathname;

  return !updatedPath.startsWith('/') ? `/${updatedPath}` : updatedPath;
};
