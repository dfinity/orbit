import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';
import { icAgent } from '~/core/IcAgent';
import { idlFactory } from '~/generated/control-panel';
import {
  Account,
  AccountBank,
  ManageAccountInput,
  RegisterAccountInput,
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

  async fetchAccount(): Promise<Account> {
    const result = await this.actor.get_account();
    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account ?? null;
  }

  async hasRegistration(): Promise<boolean> {
    return await this.fetchAccount()
      .then(_ => true)
      .catch(() => false);
  }

  async registerWithSharedBank(): Promise<Account> {
    return this.register({
      bank: {
        SharedBank: null,
      },
      name: [],
    });
  }

  async register(input: RegisterAccountInput): Promise<Account> {
    const result = await this.actor.register_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async editAccount(input: ManageAccountInput): Promise<Account> {
    const result = await this.actor.manage_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async getMainBank(): Promise<Maybe<AccountBank>> {
    const result = await this.actor.get_main_bank();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.bank?.[0] ?? null;
  }

  async listBanks(): Promise<AccountBank[]> {
    const result = await this.actor.list_banks();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.banks;
  }
}
