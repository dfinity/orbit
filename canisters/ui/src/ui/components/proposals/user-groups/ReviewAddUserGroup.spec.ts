import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import ReviewAddUserGroup from './ReviewAddUserGroup.vue';
import { AddUserGroupOperation, Proposal } from '~/generated/wallet/wallet.did';

describe('ListItemAddUserGroup', () => {
  it('renders properly', () => {
    const wrapper = mount(ReviewAddUserGroup, {
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
    const wrapper = mount(ReviewAddUserGroup, {
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

    const nameInput = wrapper.find('[name="name"]');
    expect(nameInput.exists()).toBe(true);

    const inputElement = nameInput.element as HTMLInputElement;
    expect(inputElement.value).toBe('test');
  });
});
