/* eslint-disable vue/one-component-per-file */
import { flushPromises } from '@vue/test-utils';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { defineComponent, h, ref } from 'vue';
import { RequestPolicyRule } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { mount } from '~/test.utils';
import { useUserGroupsAutocomplete } from './autocomplete.composable';
import { useRuleToTooltip } from './request-policies.composable';
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

const TooltipReader = defineComponent({
  setup() {
    useRuleToTooltip(
      ref<RequestPolicyRule>({ Quorum: { min_approved: 1, approvers: { Group: ['group-1'] } } }),
    );

    return () => h('div');
  },
});

const TooltipReviewScope = defineComponent({
  setup() {
    provideVerifiedReads();

    return () => h(TooltipReader);
  },
});

const mockUserGroup = () =>
  vi.spyOn(services().station, 'getUserGroup').mockResolvedValue({
    user_group: { id: 'group-1', name: 'Group 1' },
    privileges: { id: 'group-1', can_edit: false, can_delete: false },
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

  it('uses query calls for rule tooltips outside of a review scope', async () => {
    mockUserGroup();

    mount(TooltipReader);
    await flushPromises();

    expect(services().station.getUserGroup).toHaveBeenCalledWith(
      { user_group_id: 'group-1' },
      false,
    );
  });

  it('uses verified calls for rule tooltips inside a review scope', async () => {
    mockUserGroup();

    mount(TooltipReviewScope);
    await flushPromises();

    expect(services().station.getUserGroup).toHaveBeenCalledWith(
      { user_group_id: 'group-1' },
      true,
    );
  });
});
