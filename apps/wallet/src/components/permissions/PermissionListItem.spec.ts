import { describe, expect, it, vi } from 'vitest';
import { ResourceTypeEnum } from '~/types/permissions.types';
import { mount } from '~/test.utils';
import PermissionListItem from './PermissionListItem.vue';

describe('PermissionListItem', () => {
  it('renders properly', () => {
    const wrapper = mount(PermissionListItem, {
      props: {
        resource: {
          match: vi.fn().mockReturnValue(false),
          resources: [],
          resourceType: ResourceTypeEnum.Permission,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the mobile version if the screen is set to mobile', () => {
    const wrapper = mount(
      PermissionListItem,
      {
        props: {
          resource: {
            match: vi.fn().mockReturnValue(false),
            resources: [],
            resourceType: ResourceTypeEnum.Permission,
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
      PermissionListItem,
      {
        props: {
          resource: {
            match: vi.fn().mockReturnValue(false),
            resources: [],
            resourceType: ResourceTypeEnum.Permission,
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
