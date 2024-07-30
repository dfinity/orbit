import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { mount } from '~/test.utils';
import DisasterRecoveryForm from './DisasterRecoveryForm.vue';
import { StationService } from '~/services/station.service';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listUserGroups: vi.fn().mockImplementation(() =>
      Promise.resolve({
        user_groups: [],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('DisasterRecoveryForm', () => {
  it('renders properly', () => {
    const wrapper = mount(DisasterRecoveryForm, {
      props: {
        modelValue: {
          quorum: 1,
          user_group_id: undefined,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('submits the form and emits it', async () => {
    const wrapper = mount(DisasterRecoveryForm, {
      props: {
        modelValue: {
          quorum: 1,
          user_group_id: undefined,
        },
      },
    });

    const quorumInput = wrapper.find('input[name="quorum"]');
    expect(quorumInput.exists()).toBe(true);
    await quorumInput.setValue('2');

    const groups = wrapper.findComponent({ name: 'VAutocomplete' });
    await groups.setValue('000-001');

    await wrapper.vm.$nextTick();

    const form = wrapper.findComponent({ ref: 'form' });
    await form.trigger('submit');

    await flushPromises();

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')).toEqual([
      [
        {
          quorum: 2,
          user_group_id: '000-001',
        },
      ],
    ]);
  });
});
