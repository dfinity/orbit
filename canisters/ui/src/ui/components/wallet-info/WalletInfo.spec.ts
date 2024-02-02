import { Principal } from '@dfinity/principal';
import { describe, expect, it, vi } from 'vitest';
import { User } from '~/generated/wallet/wallet.did';
import { services } from '~/ui/modules';
import { useSessionStore } from '~/ui/stores/session';
import { useWalletStore } from '~/ui/stores/wallet';
import { mount } from '~/ui/test.utils';
import WalletInfoVue from './WalletInfo.vue';

vi.mock('../../../services/ControlPanelService', () => ({
  ControlPanelService: vi.fn().mockImplementation(() => {
    return {
      editUser: vi.fn(() => {
        return Promise.resolve({} as User);
      }),
    };
  }),
}));

describe('WalletInfo', () => {
  function initWallet(principal: Principal, isMain: boolean, name: string | null) {
    const walletStore = useWalletStore();
    const sessionStore = useSessionStore();

    sessionStore.$patch({
      data: {
        wallets: [{ canisterId: principal.toText(), main: isMain, name }],
        selectedWallet: { canisterId: principal.toText(), hasAccess: true },
      },
    });
    walletStore.$patch({ canisterId: principal.toText() });
  }

  function addWallet(principal: Principal, isMain: boolean, name: string | null) {
    const sessionStore = useSessionStore();
    const walletStore = useWalletStore();

    sessionStore.$patch({
      data: {
        wallets: [
          ...sessionStore.data.wallets,
          { canisterId: principal.toText(), main: isMain, name },
        ],
      },
    });
    walletStore.$patch({ canisterId: principal.toText() });
  }

  function selectWallet(principal: Principal, hasAccess: boolean = true) {
    const sessionStore = useSessionStore();
    sessionStore.$patch({
      data: { selectedWallet: { canisterId: principal.toText(), hasAccess } },
    });
  }

  const walletCanisterId1 = Principal.fromUint8Array(new Uint8Array([1, 2, 3]));
  const walletCanisterId2 = Principal.fromUint8Array(new Uint8Array([1, 2, 4]));

  it('renders properly', () => {
    const wrapper = mount(WalletInfoVue);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows a remove wallet button', () => {
    const wrapper = mount(WalletInfoVue);
    expect(wrapper.find('[data-test-id="remove-wallet-btn"]').exists()).toBe(true);
  });

  it('is disabled if the wallet is the main wallet', async () => {
    const wrapper = mount(WalletInfoVue);
    initWallet(walletCanisterId1, true, 'TEST WALLET');
    await wrapper.vm.$nextTick();
    const button = wrapper.find('[data-test-id="remove-wallet-btn"]');
    expect(button.attributes('disabled')).toBeDefined();
  });

  it('is disabled if the wallet is the only wallet', async () => {
    const wrapper = mount(WalletInfoVue);
    initWallet(walletCanisterId1, false, 'TEST WALLET');
    await wrapper.vm.$nextTick();
    const button = wrapper.find('[data-test-id="remove-wallet-btn"]');
    expect(button.attributes('disabled')).toBeDefined();
  });

  it('is not disabled if the wallet is the only wallet', async () => {
    const wrapper = mount(WalletInfoVue);

    initWallet(walletCanisterId1, true, 'TEST WALLET');
    addWallet(walletCanisterId2, false, 'TEST WALLET 2');
    selectWallet(walletCanisterId2);
    await wrapper.vm.$nextTick();

    const button = wrapper.find('[data-test-id="remove-wallet-btn"]');

    expect(button.attributes('disabled')).toBeUndefined();
  });

  it('calls editUser without the removed wallet when the dialog is confirmed', async () => {
    const wrapper = mount(WalletInfoVue);

    initWallet(walletCanisterId1, true, 'TEST WALLET');
    addWallet(walletCanisterId2, false, 'TEST WALLET 2');
    selectWallet(walletCanisterId2);
    await wrapper.vm.$nextTick();
    wrapper.find('[data-test-id="remove-wallet-btn"]').trigger('click');
    await wrapper.vm.$nextTick();

    document
      .querySelector('[data-test-id="action-btn-default-submit-btn"]')
      ?.dispatchEvent(new Event('click'));

    expect(services().controlPanel.editUser).toHaveBeenCalledWith(
      expect.objectContaining({
        wallets: [
          [
            {
              canister_id: walletCanisterId1,
              name: ['TEST WALLET'],
            },
          ],
        ],
      }),
    );
  });
});
