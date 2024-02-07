import { describe, expect, it, vi } from 'vitest';
import { defaultUserSpecifiers } from '~/configs/access-policies.config';
import { ResourceActionEnum } from '~/types/access-policies.types';
import { mount } from '~/ui/test.utils';
import SpecificUsersAction from './SpecificUsersAction.vue';

describe('SpecificUsersAction', () => {
  it('renders properly', () => {
    const wrapper = mount(SpecificUsersAction, {
      props: {
        modelValue: {
          modelValue: {
            policyId: null,
            userIds: [],
            prefilledUsers: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Read,
          specifier: {
            AccessPolicy: { Create: null },
          },
          users: defaultUserSpecifiers(),
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
            policyId: null,
            userIds: [],
            prefilledUsers: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Create,
          specifier: {
            AccessPolicy: { Create: null },
          },
          users: {
            ...defaultUserSpecifiers(),
            specificUsers: {
              policy: {
                id: '1',
                canEdit: true,
                canRemove: true,
              },
              users: [],
            },
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
            policyId: null,
            userIds: [],
            prefilledUsers: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Read,
          specifier: {
            AccessPolicy: { Create: null },
          },
          users: {
            ...defaultUserSpecifiers(),
            specificUsers: {
              policy: {
                id: '1',
                canEdit: false,
                canRemove: false,
              },
              users: [],
            },
          },
        },
        submitCb: vi.fn(),
      },
    });

    const actionBtn = wrapper.find('[data-test-id="specific-users-action-btn"]');
    expect(actionBtn.exists()).toBe(false);
  });
});
