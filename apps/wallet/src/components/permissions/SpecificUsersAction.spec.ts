import { describe, expect, it, vi } from 'vitest';
import { defaultAllowLevels } from '~/configs/permissions.config';
import { mount } from '~/test.utils';
import { ResourceActionEnum } from '~/types/permissions.types';
import SpecificUsersAction from './SpecificUsersAction.vue';

describe('SpecificUsersAction', () => {
  it('renders properly', () => {
    const wrapper = mount(SpecificUsersAction, {
      props: {
        modelValue: {
          modelValue: {
            userIds: [],
            prefilledUsers: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Read,
          canEdit: true,
          resource: {
            Permission: { Update: null },
          },
          allow: defaultAllowLevels(),
        },
        submitCb: vi.fn(),
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the action btn if permission is not read only', () => {
    const wrapper = mount(SpecificUsersAction, {
      props: {
        modelValue: {
          modelValue: {
            userIds: [],
            prefilledUsers: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Create,
          resource: {
            Permission: { Update: null },
          },
          canEdit: true,
          allow: {
            ...defaultAllowLevels(),
            specificUsers: [],
          },
        },
        submitCb: vi.fn(),
      },
    });

    const actionBtn = wrapper.find('[data-test-id="specific-users-action-btn"]');
    expect(actionBtn.exists()).toBe(true);
  });

  it('hides the action btn if permission is read only', () => {
    const wrapper = mount(SpecificUsersAction, {
      props: {
        modelValue: {
          modelValue: {
            userIds: [],
            prefilledUsers: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Read,
          resource: {
            Permission: { Update: null },
          },
          canEdit: false,
          allow: {
            ...defaultAllowLevels(),
            specificUsers: [],
          },
        },
        submitCb: vi.fn(),
      },
    });

    const actionBtn = wrapper.find('[data-test-id="specific-users-action-btn"]');
    expect(actionBtn.exists()).toBe(false);
  });
});
