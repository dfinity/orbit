import { describe, expect, it } from 'vitest';
import { Request } from '~/generated/station/station.did';
import { mount } from '~/test.utils';
import RequestMetadata from './RequestMetadata.vue';

describe('RequestMetadata', () => {
  it('renders properly', () => {
    const wrapper = mount(RequestMetadata, {
      props: {
        details: { can_approve: false, requester_name: undefined, approvers: [] },
        request: {
          status: { Approved: null },
        } as Request,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the expiration_dt when still pending', () => {
    const wrapper = mount(RequestMetadata, {
      props: {
        details: { can_approve: false, requester_name: undefined, approvers: [] },
        request: {
          expiration_dt: new Date().toISOString(),
          status: { Created: null },
        } as Request,
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-test-id="expiration-dt"]').exists()).toBe(true);
  });

  it('hides the expiration_dt when not pending', () => {
    const wrapper = mount(RequestMetadata, {
      props: {
        details: { can_approve: false, requester_name: undefined, approvers: [] },
        request: {
          expiration_dt: new Date().toISOString(),
          status: { Approved: null },
        } as Request,
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-test-id="expiration-dt"]').exists()).toBe(false);
  });
});
