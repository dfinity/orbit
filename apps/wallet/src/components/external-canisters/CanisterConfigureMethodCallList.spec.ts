import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterConfigureMethodCallList from './CanisterConfigureMethodCallList.vue';

describe('CanisterConfigureMethodCallList', () => {
  it('default shows empty list', () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterConfigureMethodCallList, {
      props: {
        canisterId,
      },
    });

    const noMethodSection = form.find('[data-test-id="empty-method-list"]');

    expect(noMethodSection.exists()).toBe(true);
  });

  it('shows method list', async () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterConfigureMethodCallList, {
      props: {
        canisterId,
        permissions: [
          {
            execution_method: 'test',
            validation_method: { No: null },
            allow: {
              auth_scope: { Restricted: null },
              user_groups: [],
              users: [],
            },
          },
        ],
      },
    });

    await form.vm.$nextTick();

    const noMethodSection = form.find('[data-test-id="empty-method-list"]');
    const methodList = form.find('[data-test-id="method-list"]');

    expect(noMethodSection.exists()).toBe(false);
    expect(methodList.exists()).toBe(true);
  });
});
