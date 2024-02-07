import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ListItemAddUserGroup from './ListItemAddUserGroup.vue';
import { AddUserGroupOperation, Proposal } from '~/generated/wallet/wallet.did';

describe('ListItemAddUserGroup', () => {
  it('renders properly', () => {
    const wrapper = mount(ListItemAddUserGroup, {
      props: {
        proposal: {
          operation: {
            AddUserGroup: {
              input: {
                name: 'test',
              },
            },
          },
        } as Proposal & { operation: { AddUserGroup: AddUserGroupOperation } },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the user group name that was added', () => {
    const wrapper = mount(ListItemAddUserGroup, {
      props: {
        proposal: {
          operation: {
            AddUserGroup: {
              input: {
                name: 'test',
              },
            },
          },
        } as Proposal & { operation: { AddUserGroup: AddUserGroupOperation } },
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.text()).toBe('test');
  });
});
