import { describe, expect, it, vi } from 'vitest';
import { defaultAllowLevels } from '~/configs/permissions.config';
import { ResourceActionEnum } from '~/types/permissions.types';
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
            Permission: { Update: null },
          },
          allow: defaultAllowLevels(),
          canEdit: false,
        },
        submitCb: vi.fn(),
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the action btn if permission is not read only', () => {
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
            Permission: { Update: null },
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

  it('hides the action btn if permission is read only', () => {
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
            Permission: { Update: null },
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
