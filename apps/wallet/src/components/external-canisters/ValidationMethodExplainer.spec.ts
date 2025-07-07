import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ValidationMethodExplainer from './ValidationMethodExplainer.vue';

describe('ValidationMethodExplainer', () => {
  it('renders method and canister id', () => {
    const validationCanisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');

    const wrapper = mount(ValidationMethodExplainer, {
      props: {
        validationMethod: 'test',
        validationCanisterId,
        selfCanisterId: undefined,
      },
    });

    expect(wrapper.find('[data-test-id="target-method"] .d-none').text()).toEqual('test');
    expect(wrapper.find('[data-test-id="target-canister"]').text()).toEqual(
      validationCanisterId.toText(),
    );
  });

  it('renders method and self canister id', () => {
    const validationCanisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');

    const wrapper = mount(ValidationMethodExplainer, {
      props: {
        validationMethod: 'test',
        validationCanisterId,
        selfCanisterId: validationCanisterId,
      },
    });

    expect(wrapper.find('[data-test-id="target-method"] .d-none').text()).toEqual('test');
    expect(wrapper.find('[data-test-id="target-canister"]').text()).toEqual('self');
  });
});
