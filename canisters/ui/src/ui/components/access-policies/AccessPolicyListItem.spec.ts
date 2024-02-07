import { describe, expect, it, vi } from 'vitest';
import { ResourceTypeEnum } from '~/types/access-policies.types';
import { mount } from '~/ui/test.utils';
import AccessPolicyListItem from './AccessPolicyListItem.vue';

describe('AccessPolicyListItem', () => {
  it('renders properly', () => {
    const wrapper = mount(AccessPolicyListItem, {
      props: {
        resource: {
          match: vi.fn().mockReturnValue(false),
          specifiers: [],
          resourceType: ResourceTypeEnum.AccessPolicy,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the mobile version if the screen is set to mobile', () => {
    const wrapper = mount(
      AccessPolicyListItem,
      {
        props: {
          resource: {
            match: vi.fn().mockReturnValue(false),
            specifiers: [],
            resourceType: ResourceTypeEnum.AccessPolicy,
          },
        },
      },
      {
        initialPiniaState: {
          app: {
            isMobile: true,
          },
        },
      },
    );

    const mobileVersion = wrapper.find('[data-test-id="mobile-list-view"]');
    expect(mobileVersion.exists()).toBe(true);
  });

  it('shows the desktop version if the screen is not set to mobile', () => {
    const wrapper = mount(
      AccessPolicyListItem,
      {
        props: {
          resource: {
            match: vi.fn().mockReturnValue(false),
            specifiers: [],
            resourceType: ResourceTypeEnum.AccessPolicy,
          },
        },
      },
      {
        initialPiniaState: {
          app: {
            isMobile: false,
          },
        },
      },
    );

    const desktopVersion = wrapper.find('[data-test-id="mobile-list-view"]');
    expect(desktopVersion.exists()).toBe(false);
  });
});
