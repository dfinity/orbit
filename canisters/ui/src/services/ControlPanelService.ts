import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';
import { icAgent } from '~/core/IcAgent';
import { idlFactory } from '~/generated/control-panel';
import {
  User,
  UserWallet,
  ManageUserInput,
  RegisterUserInput,
  _SERVICE,
} from '~/generated/control-panel/control_panel.did';
import { Maybe } from '~/types';

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

  async registerWithSharedWallet(): Promise<User> {
    return this.register({
      wallet: {
        SharedWallet: null,
      },
      name: [],
    });
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
}
