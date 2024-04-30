import { describe, expect, it } from 'vitest';
import { setupComponent } from '../test.utils';
import { useAutocomplete } from './autocomplete.composable';

describe('Autocomplete Composables', () => {
  it('should be initialized with no results', () => {
    const vm = setupComponent(() => {
      return {
        autocomplete: useAutocomplete<string>((search: string) => Promise.resolve([search])),
      };
    });

    expect(vm.autocomplete.results.value.length).toBe(0);
  });

  it('should load results on search items call', async () => {
    const vm = setupComponent(() => {
      return {
        autocomplete: useAutocomplete<string>((search: string) => Promise.resolve([search])),
      };
    });

    await vm.autocomplete.searchItems('test');

    expect(vm.autocomplete.results.value).toEqual(['test']);
  });
});
