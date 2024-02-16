import { describe, expect, it } from 'vitest';
import { Proposal } from '~/generated/wallet/wallet.did';
import { mount } from '~/test.utils';
import ReviewAddUserGroup from './ReviewAddUserGroup.vue';

describe('ListItemAddUserGroup', () => {
  it('renders properly', () => {
    const wrapper = mount(ReviewAddUserGroup, {
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
    const wrapper = mount(ReviewAddUserGroup, {
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

    const nameInput = wrapper.find('[name="name"]');
    expect(nameInput.exists()).toBe(true);

    const inputElement = nameInput.element as HTMLInputElement;
    expect(inputElement.value).toBe('test');
  });
});
