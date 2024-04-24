import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { appInitConfig } from '~/configs/init.config';
import { idlFactory } from '~/generated/control-panel';
import {
  ManageUserInput,
  RegisterUserInput,
  User,
  UserWallet,
  _SERVICE,
} from '~/generated/control-panel/control_panel.did';
import { Maybe } from '~/types/helper.types';
import { variantIs } from '~/utils/helper.utils';

export class ControlPanelService {
  private actor: ActorSubclass<_SERVICE>;

  constructor(agent: HttpAgent) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent,
      canisterId: appInitConfig.canisters.controlPanel,
    });
  }

  async getCurrentUser(): Promise<User> {
    const result = await this.actor.get_user();
    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async subscribeToWaitlist(email: string): Promise<void> {
    const result = await this.actor.subscribe_to_waiting_list(email);

    if ('Err' in result) {
      throw result.Err;
    }
  }

  async hasRegistration(): Promise<boolean> {
    return await this.getCurrentUser()
      .then(_ => true)
      .catch(() => false);
  }

  async register(input: RegisterUserInput): Promise<User> {
    const result = await this.actor.register_user(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async setUserActive(): Promise<void> {
    const result = await this.actor.set_user_active();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }
  }

  async editUser(input: ManageUserInput): Promise<User> {
    const result = await this.actor.manage_user(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async getMainWallet(): Promise<Maybe<UserWallet>> {
    const result = await this.actor.get_main_wallet();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.wallet?.[0] ?? null;
  }

  async listWallets(): Promise<UserWallet[]> {
    const result = await this.actor.list_wallets();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.wallets;
  }

  async deployWallet(): Promise<Principal> {
    const result = await this.actor.deploy_wallet();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.canister_id;
  }
}
