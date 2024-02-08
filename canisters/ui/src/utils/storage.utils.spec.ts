import { describe } from 'node:test';
import { afterEach, expect, it, vi } from 'vitest';
import { setupComponent } from '~/test.utils';
import { objectDeserialize, objectSerialize, useStorage } from './storage.utils';
import logger from '~/core/logger.core';

const setItem = vi.spyOn(Storage.prototype, 'setItem');
const getItem = vi.spyOn(Storage.prototype, 'getItem');

describe('storage', () => {
  afterEach(() => {
    setItem.mockClear();
    getItem.mockClear();

    global.localStorage.clear();
  });

  it(`should try reading the existing value`, () => {
    setupComponent(() => ({
      useStorage: useStorage({
        initial: () => ({ key: 'initial-value' }),
        key: 'test',
        storage: global.localStorage,
        serialize: objectSerialize,
        deserialize: objectDeserialize,
      }),
    }));
    expect(getItem).toHaveBeenCalledWith('test');
  });

  it(`should return a ref with the initial value if the key doesn't exist`, () => {
    const component = setupComponent(() => ({
      useStorage: useStorage({
        initial: () => ({ key: 'initial-value' }),
        key: 'test',
        storage: global.localStorage,
        serialize: objectSerialize,
        deserialize: objectDeserialize,
      }),
    }));
    expect(component.useStorage).toEqual({ key: 'initial-value' });
  });

  it(`should return a ref with the stored value if the key exists`, () => {
    global.localStorage.setItem('test', objectSerialize({ key: 'stored-value' }));

    const component = setupComponent(() => ({
      useStorage: useStorage({
        initial: () => ({ key: 'initial-value' }),
        key: 'test',
        storage: global.localStorage,
        serialize: objectSerialize,
        deserialize: objectDeserialize,
      }),
    }));
    expect(component.useStorage).toEqual({ key: 'stored-value' });
  });

  it(`should fall back to the initial value if deserialization fails`, () => {
    global.localStorage.setItem('test', 'invalid-json');

    // this is to suppress the warning in the console output when the test runs
    vi.spyOn(logger, 'warn').mockImplementation(() => {});

    const component = setupComponent(() => ({
      useStorage: useStorage({
        initial: () => ({ key: 'initial-value' }),
        key: 'test',
        storage: global.localStorage,
        serialize: objectSerialize,
        deserialize: objectDeserialize,
      }),
    }));
    expect(component.useStorage).toEqual({ key: 'initial-value' });
  });

  it(`should persist the initial value to the storage`, () => {
    expect(global.localStorage.getItem('test')).toBeNull();
    setupComponent(() => ({
      useStorage: useStorage({
        initial: () => ({ key: 'initial-value' }),
        key: 'test',
        storage: global.localStorage,
        serialize: objectSerialize,
        deserialize: objectDeserialize,
      }),
    }));
    expect(setItem).toHaveBeenCalledOnce();
    expect(setItem).toHaveBeenCalledWith('test', objectSerialize({ key: 'initial-value' }));
  });

  it(`should persist value changes to the storage`, async () => {
    const component = setupComponent(() => ({
      useStorage: useStorage({
        initial: () => ({ key: 'initial-value' }),
        key: 'test',
        storage: global.localStorage,
        serialize: objectSerialize,
        deserialize: objectDeserialize,
      }),
    }));

    setItem.mockClear();

    component.useStorage = {
      key: 'changed-value',
    };
    await component.$nextTick();

    expect(setItem).toHaveBeenCalledOnce();
    expect(setItem).toHaveBeenCalledWith('test', objectSerialize({ key: 'changed-value' }));
  });
});
