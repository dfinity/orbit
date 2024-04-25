import { Principal } from '@dfinity/principal';
import { UserWallet } from '~/generated/control-panel/control_panel.did';
import { UserWallet as SessionUserWallet } from '~/stores/session.store';

export function sessionUserWalletToUserWallet(wallet: Omit<SessionUserWallet, 'main'>): UserWallet {
  return {
    canister_id: Principal.fromText(wallet.canisterId),
    name: wallet.name ? [wallet.name] : [],
  };
}
