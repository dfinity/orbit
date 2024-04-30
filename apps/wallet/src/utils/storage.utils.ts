import { Ref, ref, watch } from 'vue';
import { logger } from '~/core/logger.core';

export const objectSerialize = <T>(value: T): string => JSON.stringify(value);
export const objectDeserialize = <T>(value: string): T => JSON.parse(value);

export function useStorage<T>({
  key,
  storage,
  initial,
  serialize,
  deserialize,
  deepWatch = true,
}: {
  key: string;
  storage: Storage;
  initial: () => T;
  serialize: (value: T) => string;
  deserialize: (value: string) => T;
  deepWatch?: boolean;
}): Ref<T> {
  const storedValue: string | null = storage.getItem(key);
  const valueIsStored = storedValue !== null;

  let initialValue: T;

  if (valueIsStored) {
    try {
      initialValue = deserialize(storedValue);
    } catch (error) {
      logger.warn(`Failed to deserialize value for key ${key}: ${storedValue}`, error);
      initialValue = initial();
    }
  } else {
    initialValue = initial();
  }

  const data = ref(initialValue) as Ref<T>;

  if (!valueIsStored) {
    storage.setItem(key, serialize(initialValue));
  }

  watch(
    data,
    newValue => {
      storage.setItem(key, serialize(newValue));
    },
    { deep: deepWatch },
  );

  return data;
}
