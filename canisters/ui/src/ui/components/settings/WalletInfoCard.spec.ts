import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import { useSessionStore } from '~/ui/stores/session';
import { mount } from '~/ui/test.utils';
import WalletInfoCard from './WalletInfoCard.vue';

describe('WalletInfoCard', () => {
  it('renders properly', () => {
    const wrapper = mount(WalletInfoCard);

    expect(wrapper.exists()).toBe(true);
  });

  it('renders the wallet name', async () => {
    const wrapper = mount(WalletInfoCard);

    const session = useSessionStore();
    session.data.wallets = [
      {
        canisterId: Principal.anonymous().toText(),
        main: true,
        name: 'Personal',
      },
    ];

    await wrapper.vm.$nextTick();

    const nameLine = wrapper.find('[data-test-id="wallet-name"]');

    expect(nameLine.exists()).toBeTruthy();
    expect(nameLine.text()).toContain('Personal');
  });
});
