import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';
import { icAgent } from '~/core/IcAgent';
import { idlFactory } from '~/generated/control-panel';
import {
  User,
  UserBank,
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

  async registerWithSharedBank(): Promise<User> {
    return this.register({
      bank: {
        SharedBank: null,
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

  async getMainBank(): Promise<Maybe<UserBank>> {
    const result = await this.actor.get_main_bank();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.bank?.[0] ?? null;
  }

  async listBanks(): Promise<UserBank[]> {
    const result = await this.actor.list_banks();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.banks;
  }
}
