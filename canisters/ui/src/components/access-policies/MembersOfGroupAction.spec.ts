import { describe, expect, it, vi } from 'vitest';
import { defaultAllowLevels } from '~/configs/access-policies.config';
import { ResourceActionEnum } from '~/types/access-policies.types';
import { mount } from '~/test.utils';
import MembersOfGroupAction from './MembersOfGroupAction.vue';

describe('MembersOfGroupAction', () => {
  it('renders properly', () => {
    const wrapper = mount(MembersOfGroupAction, {
      props: {
        modelValue: {
          modelValue: {
            groupIds: [],
            prefilledGroups: [],
          },
          valid: false,
        },
        specifier: {
          action: ResourceActionEnum.Read,
          resource: {
            AccessPolicy: { Edit: { Any: null } },
          },
          allow: defaultAllowLevels(),
          canEdit: false,
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
            groupIds: [],
            prefilledGroups: [],
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
            membersOfGroup: [],
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
            groupIds: [],
            prefilledGroups: [],
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
            membersOfGroup: [],
          },
        },
        submitCb: vi.fn(),
      },
    });

    const actionBtn = wrapper.find('[data-test-id="members-of-group-action-btn"]');
    expect(actionBtn.exists()).toBe(false);
  });
});
