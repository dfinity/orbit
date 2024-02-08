import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { appInitConfig } from '~/configs/init.config';
import { icAgent } from '~/core/ic-agent.core';
import { idlFactory } from '~/generated/control-panel';
import {
  ManageUserInput,
  RegisterUserInput,
  User,
  UserWallet,
  _SERVICE,
} from '~/generated/control-panel/control_panel.did';
import { Maybe } from '~/types/helper.types';

export class ControlPanelService {
  private actor: ActorSubclass<_SERVICE>;

  constructor(agent: HttpAgent = icAgent.get()) {
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

    return result.Ok.user ?? null;
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
