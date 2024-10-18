import { Certificate, HttpAgent, LookupStatus } from '@dfinity/agent';
import type { IDL as CandidIDL } from '@dfinity/candid';
import { Principal } from '@dfinity/principal';
import { toRaw } from 'vue';
import { LocationQuery, LocationQueryValue } from 'vue-router';
import { TransferStatus } from '~/generated/station/station.did';
import { AccountTransferStatus } from '~/types/station.types';
import { arrayBufferToHex } from '~/utils/crypto.utils';

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

export const toArrayBuffer = (input: Uint8Array | number[]): ArrayBuffer => {
  return input instanceof Uint8Array ? input.buffer : new Uint8Array(input).buffer;
};

export const toUint8Array = (input: ArrayBuffer | number[] | Uint8Array): Uint8Array => {
  if (input instanceof Uint8Array) {
    return input;
  }

  return new Uint8Array(input);
};

/**
 * Removes all null and undefined values from an array and returns a new array.
 *
 * @param array - The array to compact.
 * @returns A new array with all null and undefined values removed.
 */
export const compactArray = <T, R = T>(
  array: (T | null | undefined)[],
  opts: {
    /** If true, also removes empty strings from the array. */
    removeEmptyStrings?: boolean;
    /** if provided, only includes items that are in the set. */
    include?: Set<unknown>;
  } = {},
): R[] => {
  return array.filter(item => {
    if (item === null || item === undefined) {
      return false;
    }

    if (opts.removeEmptyStrings && item === '') {
      return false;
    }

    if (opts.include && !opts.include.has(item)) {
      return false;
    }

    return true;
  }) as R[];
};

/**
 * Parses a location query object and returns a record with string arrays.
 *
 * Removes all null and undefined values from the query object.
 *
 * @param query The location query object.
 * @returns a record with string arrays.
 */
export const parseLocationQuery = (query: LocationQuery): Record<string, string[]> => {
  const result: Record<string, string[]> = {};

  for (const key in query) {
    if (typeof query[key] === 'string' && query[key] !== '') {
      result[key] = [query[key] as string];
    } else if (Array.isArray(query[key]) && query[key]?.length) {
      result[key] = compactArray<string>(query[key] as LocationQueryValue[], {
        removeEmptyStrings: true,
      });
    }
  }

  return result;
};

/**
 * Parses a value to a BigInt or returns undefined if the value is not a valid BigInt.
 *
 * @param value The value to parse.
 * @returns The parsed BigInt value or undefined if the value is not a valid BigInt.
 */
export const parseToBigIntOrUndefined = (
  value: string | number | bigint | null | undefined,
): bigint | undefined => {
  try {
    if (value === undefined || value === null) {
      return undefined;
    }

    if (typeof value === 'bigint') {
      return value;
    }

    if (typeof value === 'string') {
      return value.trim() !== '' ? BigInt(value) : undefined;
    }

    return BigInt(value);
  } catch (error) {
    return undefined;
  }
};

export const parseToNumberOrUndefined = (
  value: string | number | bigint | null | undefined,
): number | undefined => {
  if (value === undefined || value === null) {
    return undefined;
  }

  if (typeof value === 'number') {
    return value;
  }

  if (typeof value === 'string') {
    return value.trim() !== '' ? Number(value) : undefined;
  }

  if (typeof value === 'bigint') {
    return Number(value);
  }

  return Number(value);
};

export async function fetchCanisterModuleHash(
  agent: HttpAgent,
  canisterId: Principal,
): Promise<string | null> {
  const encoder = new TextEncoder();
  const moduleHashPath: ArrayBuffer[] = [
    encoder.encode('canister'),
    canisterId.toUint8Array(),
    encoder.encode('module_hash'),
  ];

  const state = await agent.readState(canisterId, {
    paths: [moduleHashPath],
  });

  const certificate = await Certificate.create({
    canisterId,
    certificate: state.certificate,
    rootKey: agent.rootKey,
  });

  const moduleHash = certificate.lookup(moduleHashPath);

  if (moduleHash.status !== LookupStatus.Found) {
    return null;
  }

  if (!(moduleHash.value instanceof ArrayBuffer)) {
    throw new Error('Module hash value is not an ArrayBuffer');
  }

  return arrayBufferToHex(moduleHash.value);
}

/**
 * Transforms the input data to a new value where all complex objects are transformed to a JSON serializable format.
 *
 * @param input - The input data to transform.
 * @param opts - The options for the transformation.
 *
 * @param opts.removeUndefinedOrNull - If true, removes all undefined and null values from the input.
 * @param opts.removeEmptyArrays - If true, removes all empty arrays from the input.
 * @param opts.removeFunctions - If true, removes all functions from the input.
 * @param opts.transformBufferAsHex - If true, transforms all ArrayBuffer values to hex strings.
 *
 * @returns The transformed data.
 */
export const transformData = (
  input: unknown,
  opts: {
    removeUndefinedOrNull?: boolean;
    removeEmptyLists?: boolean;
    removeFunctions?: boolean;
    transformBufferAsHex?: boolean;
  } = {},
): unknown => {
  const {
    removeEmptyLists = false,
    removeUndefinedOrNull = true,
    removeFunctions = true,
    transformBufferAsHex = true,
  } = opts;

  const seen = new WeakSet();
  const normalize = (data: unknown): unknown => {
    // Handles circular references by returning a string '[Circular Reference]' when a circular reference is found.
    if (typeof data === 'object' && data !== null) {
      if (seen.has(data)) {
        return '[Circular Reference]';
      }

      seen.add(data);
    }

    if (data === null || data === undefined) {
      return removeUndefinedOrNull ? undefined : data;
    }

    if (typeof data === 'function') {
      return !removeFunctions ? '[Function]' : undefined;
    }

    if (typeof data === 'bigint') {
      return Number(data);
    }

    if (data instanceof Principal) {
      return data.toText();
    }

    if (data instanceof Date) {
      return data.toISOString();
    }

    if (data instanceof ArrayBuffer) {
      if (removeEmptyLists && data.byteLength === 0) {
        return undefined;
      }

      return transformBufferAsHex ? arrayBufferToHex(data) : Array.from(new Uint8Array(data));
    }

    if (data instanceof Uint8Array) {
      if (removeEmptyLists && data.length === 0) {
        return undefined;
      }

      return Array.from(data);
    }

    if (Array.isArray(data)) {
      if (removeEmptyLists && data.length === 0) {
        return undefined;
      }

      return data
        .map(value => normalize(value))
        .filter(data => (removeUndefinedOrNull ? data !== undefined : true));
    }

    if (data instanceof Map) {
      const result: Record<string, unknown> = {};

      data.forEach((value, key) => {
        if (removeUndefinedOrNull && (value === null || value === undefined)) {
          return;
        }

        result[key] = normalize(value);
      });

      if (removeEmptyLists && Object.keys(result).length === 0) {
        return undefined;
      }

      return result;
    }

    if (data instanceof Set) {
      if (removeEmptyLists && data.size === 0) {
        return undefined;
      }

      return Array.from(data)
        .map(value => normalize(value))
        .filter(data => (removeUndefinedOrNull ? data !== undefined : true));
    }

    if (data instanceof Object) {
      const result: Record<string, unknown> = {};

      Object.entries(data).forEach(([key, value]) => {
        if (removeUndefinedOrNull && (value === null || value === undefined)) {
          return;
        }

        result[key] = normalize(value);
      });

      if (removeEmptyLists && Object.keys(result).length === 0) {
        return undefined;
      }

      return result;
    }

    return data;
  };

  const normalizedInput = normalize(input);

  if (typeof normalizedInput === 'object' || Array.isArray(normalizedInput)) {
    const plainJson = JSON.parse(
      JSON.stringify(normalizedInput, (_, value) => {
        // Json stringify takes care of removing all keys with undefined values, so we only need to remove null values.
        if (removeUndefinedOrNull && value === null) {
          return undefined;
        }

        return value;
      }),
    );

    return plainJson;
  }

  return normalizedInput;
};

/**
 * Deep clones the input data using structured cloning, if Proxy objects are found they are
 * transformed to plain objects.
 */
export function deepClone<T>(input: T): T {
  const value = toRaw(input);

  if (Array.isArray(value)) {
    return value.map(deepClone) as T;
  }

  if (value === null) return null as T;

  if (value instanceof Principal) {
    return Principal.fromUint8Array(value.toUint8Array()) as T;
  }

  if (typeof value === 'object') {
    const entries = Object.entries(value).map(([key, value]) => [key, deepClone(value)]);

    return Object.fromEntries(entries);
  }

  return structuredClone(value);
}

/**
 * Debounces a function and returns a promise that resolves with the result of the debounced function.
 *
 * @param debouncedFunction The function to debounce.
 * @param wait The time to wait before calling the debounced function. @default 500
 *
 * @returns The debounced function.
 */
export const debounce = <T extends (...args: Parameters<T>) => ReturnType<T>>(
  debouncedFunction: T,
  wait: number = 500,
  opts: { immediate?: boolean | number } = {},
): ((...args: Parameters<T>) => Promise<Awaited<ReturnType<T>>>) => {
  let timeout: NodeJS.Timeout | null = null;
  let isFirstCall = true;
  const immediateMs =
    (typeof opts.immediate === 'number' && opts.immediate >= 0) || opts.immediate === true
      ? Number(opts.immediate)
      : false;

  return (...args: Parameters<T>): Promise<Awaited<ReturnType<T>>> => {
    const waitMs = isFirstCall && immediateMs !== false ? immediateMs : wait;
    isFirstCall = false;

    return new Promise((resolve, reject) => {
      if (timeout) {
        clearTimeout(timeout);
      }

      timeout = setTimeout(() => {
        try {
          const result = debouncedFunction(...args);

          return Promise.resolve(result).then(resolve).catch(reject);
        } catch (error) {
          reject(error);
        }
      }, waitMs);
    });
  };
};
