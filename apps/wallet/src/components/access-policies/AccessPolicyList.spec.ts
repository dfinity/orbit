import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import AccessPolicyList from './AccessPolicyList.vue';

describe('AccessPolicyList', () => {
  it('renders properly', () => {
    const wrapper = mount(AccessPolicyList, {
      props: {
        accessPolicies: [],
        privileges: [],
        resources: [],
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('hides extra headers when screen is set to mobile', () => {
    const wrapper = mount(
      AccessPolicyList,
      {
        props: {
          accessPolicies: [],
          privileges: [],
          resources: [],
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

    const mobileHeaders = wrapper.find('[data-test-id="mobile-table-headers"]');
    expect(mobileHeaders.exists()).toBe(true);
  });

  it('shows extra headers when screen is not set to mobile', () => {
    const wrapper = mount(
      AccessPolicyList,
      {
        props: {
          accessPolicies: [],
          privileges: [],
          resources: [],
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

    const mobileHeaders = wrapper.find('[data-test-id="mobile-table-headers"]');
    expect(mobileHeaders.exists()).toBe(false);
  });
});
