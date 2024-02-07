import { describe, expect, it, vi } from 'vitest';
import { defaultUserSpecifiers } from '~/configs/access-policies.config';
import { ResourceActionEnum } from '~/types/access-policies.types';
import { mount } from '~/test.utils';
import MembersOfGroupAction from './MembersOfGroupAction.vue';

describe('MembersOfGroupAction', () => {
  it('renders properly', () => {
    const wrapper = mount(MembersOfGroupAction, {
      props: {
        modelValue: {
          modelValue: {
            policyId: null,
            groupIds: [],
            prefilledGroups: [],
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
    const wrapper = mount(MembersOfGroupAction, {
      props: {
        modelValue: {
          modelValue: {
            policyId: null,
            groupIds: [],
            prefilledGroups: [],
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
            membersOfGroup: {
              policy: {
                id: '1',
                canEdit: true,
                canRemove: true,
              },
              groups: [],
            },
          },
        },
        submitCb: vi.fn(),
      },
    });

    const actionBtn = wrapper.find('[data-test-id="members-of-group-action-btn"]');
    expect(actionBtn.exists()).toBe(true);
  });

  it('hides the action btn if access policy is read only', () => {
    const wrapper = mount(MembersOfGroupAction, {
      props: {
        modelValue: {
          modelValue: {
            policyId: null,
            groupIds: [],
            prefilledGroups: [],
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
            membersOfGroup: {
              policy: {
                id: '1',
                canEdit: false,
                canRemove: false,
              },
              groups: [],
            },
          },
        },
        submitCb: vi.fn(),
      },
    });

    const actionBtn = wrapper.find('[data-test-id="members-of-group-action-btn"]');
    expect(actionBtn.exists()).toBe(false);
  });
});
