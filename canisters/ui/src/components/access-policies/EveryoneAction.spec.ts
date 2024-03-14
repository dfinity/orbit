import { describe, expect, it } from 'vitest';
import { defaultAllowLevels } from '~/configs/access-policies.config';
import { AccessPolicyForAllUsers, ResourceActionEnum } from '~/types/access-policies.types';
import { mount } from '~/test.utils';
import EveryoneAction from './EveryoneAction.vue';

describe('MembersOfGroupAction', () => {
  it('renders properly', () => {
    const wrapper = mount(EveryoneAction, {
      props: {
        specifier: {
          action: ResourceActionEnum.Read,
          resource: {
            AccessPolicy: { Edit: { Any: null } },
          },
          allow: defaultAllowLevels(),
          canEdit: true,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the action btn if access policy is not read only', () => {
    const wrapper = mount(
      EveryoneAction,
      {
        props: {
          specifier: {
            action: ResourceActionEnum.Create,
            resource: {
              AccessPolicy: { Edit: { Any: null } },
            },
            canEdit: true,
            allow: {
              ...defaultAllowLevels(),
              allUsers: AccessPolicyForAllUsers.AuthenticationRequired,
            },
          },
        },
      },
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
        },
      },
    );

    const actionBtn = wrapper.find('[data-test-id="everyone-action-btn"]');
    expect(actionBtn.exists()).toBe(true);
  });

  it('hides the action btn if access policy is read only', () => {
    const wrapper = mount(EveryoneAction, {
      props: {
        specifier: {
          action: ResourceActionEnum.Read,
          resource: {
            AccessPolicy: { Edit: { Any: null } },
          },
          canEdit: true,
          allow: {
            ...defaultAllowLevels(),
            allUsers: AccessPolicyForAllUsers.AuthenticationRequired,
          },
        },
      },
    });

    const actionBtn = wrapper.find('[data-test-id="everyone-action-btn"]');
    expect(actionBtn.exists()).toBe(false);
  });
});
