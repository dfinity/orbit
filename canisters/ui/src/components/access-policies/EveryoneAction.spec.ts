import { describe, expect, it } from 'vitest';
import { defaultUserSpecifiers } from '~/configs/access-policies.config';
import { ResourceActionEnum } from '~/types/access-policies.types';
import { mount } from '~/test.utils';
import EveryoneAction from './EveryoneAction.vue';

describe('MembersOfGroupAction', () => {
  it('renders properly', () => {
    const wrapper = mount(EveryoneAction, {
      props: {
        specifier: {
          action: ResourceActionEnum.Read,
          specifier: {
            AccessPolicy: { Create: null },
          },
          users: defaultUserSpecifiers(),
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
            specifier: {
              AccessPolicy: { Create: null },
            },
            users: {
              ...defaultUserSpecifiers(),
              allUsers: {
                policy: {
                  id: '1',
                  canEdit: true,
                  canRemove: true,
                },
              },
            },
          },
        },
      },
      {
        initialPiniaState: {
          wallet: { privileges: [{ AddAccessPolicy: null }] },
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
          specifier: {
            AccessPolicy: { Create: null },
          },
          users: {
            ...defaultUserSpecifiers(),
            allUsers: {
              policy: {
                id: '1',
                canEdit: false,
                canRemove: false,
              },
            },
          },
        },
      },
    });

    const actionBtn = wrapper.find('[data-test-id="everyone-action-btn"]');
    expect(actionBtn.exists()).toBe(false);
  });
});
