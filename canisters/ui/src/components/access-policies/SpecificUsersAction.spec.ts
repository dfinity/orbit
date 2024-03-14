import { describe, expect, it, vi } from 'vitest';
import { defaultAllowLevels } from '~/configs/access-policies.config';
import { mount } from '~/test.utils';
import { ResourceActionEnum } from '~/types/access-policies.types';
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
            AccessPolicy: { Edit: { Any: null } },
          },
          allow: defaultAllowLevels(),
        },
        submitCb: vi.fn(),
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the action btn if access policy is not read only', () => {
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
            AccessPolicy: { Edit: { Any: null } },
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

  it('hides the action btn if access policy is read only', () => {
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
            AccessPolicy: { Edit: { Any: null } },
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
