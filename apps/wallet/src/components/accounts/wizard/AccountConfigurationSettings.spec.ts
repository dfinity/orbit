import { describe } from 'node:test';
import { expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import AccountConfigurationSettings from './AccountConfigurationSettings.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AccountConfigurationSettings', () => {
  it('prefills account name', () => {
    const wrapper = mount(AccountConfigurationSettings, {
      props: {
        mode: 'edit',
        modelValue: {
          name: 'Account',
        },
      },
    });

    const nameInput = wrapper.find('input[name="name"]');
    expect(nameInput.exists()).toBe(true);

    expect((nameInput.element as HTMLInputElement).value).toBe('Account');
  });

  it('updates the account name and emits the change', async () => {
    const wrapper = mount(AccountConfigurationSettings, {
      props: {
        mode: 'edit',
        modelValue: {
          name: 'Account',
        },
      },
    });

    const nameInput = wrapper.find('input[name="name"]');
    expect(nameInput.exists()).toBe(true);

    await nameInput.setValue('Personal');

    expect((nameInput.element as HTMLInputElement).value).toBe('Personal');

    expect(wrapper.emitted('change')).toBeTruthy();
  });
});
