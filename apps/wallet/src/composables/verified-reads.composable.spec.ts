/* eslint-disable vue/one-component-per-file */
import { flushPromises } from '@vue/test-utils';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { defineComponent, h } from 'vue';
import { services } from '~/plugins/services.plugin';
import { mount } from '~/test.utils';
import { useUserGroupsAutocomplete } from './autocomplete.composable';
import { provideVerifiedReads, useVerifiedReads } from './verified-reads.composable';

const Reader = defineComponent({
  setup() {
    const verifiedReads = useVerifiedReads();
    const autocomplete = useUserGroupsAutocomplete();

    autocomplete.searchItems();

    return () => h('div', { 'data-test-id': 'verified' }, String(verifiedReads));
  },
});

const ReviewScope = defineComponent({
  setup() {
    provideVerifiedReads();

    return () => h(Reader);
  },
});

describe('Verified reads composable', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('uses query calls outside of a review scope', async () => {
    vi.spyOn(services().station, 'listUserGroups').mockResolvedValue({
      user_groups: [],
      next_offset: [],
      total: BigInt(0),
      privileges: [],
    });

    const wrapper = mount(Reader);
    await flushPromises();

    expect(wrapper.find('[data-test-id="verified"]').text()).toBe('false');
    // the autocomplete search is debounced
    await vi.waitFor(() =>
      expect(services().station.listUserGroups).toHaveBeenCalledWith(expect.anything(), false),
    );
  });

  it('uses verified calls for lookups made inside a review scope', async () => {
    vi.spyOn(services().station, 'listUserGroups').mockResolvedValue({
      user_groups: [],
      next_offset: [],
      total: BigInt(0),
      privileges: [],
    });

    const wrapper = mount(ReviewScope);
    await flushPromises();

    expect(wrapper.find('[data-test-id="verified"]').text()).toBe('true');
    // the autocomplete search is debounced
    await vi.waitFor(() =>
      expect(services().station.listUserGroups).toHaveBeenCalledWith(expect.anything(), true),
    );
  });
});
