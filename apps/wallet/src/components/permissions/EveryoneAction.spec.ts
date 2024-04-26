import { describe, expect, it } from 'vitest';
import { defaultAllowLevels } from '~/configs/permissions.config';
import { AuthScopeEnum, ResourceActionEnum } from '~/types/permissions.types';
import { mount } from '~/test.utils';
import EveryoneAction from './EveryoneAction.vue';

describe('MembersOfGroupAction', () => {
  it('renders properly', () => {
    const wrapper = mount(EveryoneAction, {
      props: {
        specifier: {
          action: ResourceActionEnum.Read,
          resource: {
            Permission: { Update: null },
          },
          allow: defaultAllowLevels(),
          canEdit: true,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the action btn if permission is not read only', () => {
    const wrapper = mount(
      EveryoneAction,
      {
        props: {
          specifier: {
            action: ResourceActionEnum.Create,
            resource: {
              Permission: { Update: null },
            },
            canEdit: true,
            allow: {
              ...defaultAllowLevels(),
              authScope: AuthScopeEnum.Authenticated,
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

  it('hides the action btn if permission is read only', () => {
    const wrapper = mount(EveryoneAction, {
      props: {
        specifier: {
          action: ResourceActionEnum.Read,
          resource: {
            Permission: { Update: null },
          },
          canEdit: false,
          allow: {
            ...defaultAllowLevels(),
            authScope: AuthScopeEnum.Authenticated,
          },
        },
      },
    });

    const actionBtn = wrapper.find('[data-test-id="everyone-action-btn"]');
    expect(actionBtn.attributes()['disabled'] !== undefined).toBeTruthy();
  });
});
