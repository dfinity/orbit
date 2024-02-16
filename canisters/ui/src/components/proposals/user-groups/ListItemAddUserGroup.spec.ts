import { describe, expect, it } from 'vitest';
import { Proposal } from '~/generated/wallet/wallet.did';
import { mount } from '~/test.utils';
import ListItemAddUserGroup from './ListItemAddUserGroup.vue';

describe('ListItemAddUserGroup', () => {
  it('renders properly', () => {
    const wrapper = mount(ListItemAddUserGroup, {
      props: {
        proposal: {
          operation: {
            AddUserGroup: {
              user_group: [],
              input: {
                name: 'test',
              },
            },
          },
        } as Proposal,
        operation: {
          user_group: [],
          input: {
            name: 'test',
          },
        },
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
              user_group: [],
              input: {
                name: 'test',
              },
            },
          },
        } as Proposal,
        operation: {
          user_group: [],
          input: {
            name: 'test',
          },
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.text()).toBe('test');
  });
});
