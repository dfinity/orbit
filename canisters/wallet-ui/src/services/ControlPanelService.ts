import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';
import { icAgent } from '~/core/IcAgent';
import { idlFactory } from '~/generated/control-panel';
import {
  Account,
  AccountDetails,
  ManageAccountInput,
  RegisterAccountInput,
  _SERVICE,
} from '~/generated/control-panel/control_panel.did';

export class ControlPanelService {
  private actor: ActorSubclass<_SERVICE>;

  constructor(agent: HttpAgent = icAgent.get()) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent,
      canisterId: appInitConfig.canisters.controlPanel,
    });
  }

  async get_account_details(): Promise<AccountDetails | null> {
    const account_details = await this.actor.account_details();
    if ('Err' in account_details) {
      throw account_details.Err;
    }

    return account_details.Ok.account_details.length ? account_details.Ok.account_details[0] : null;
  }

  async register_with_shared_bank(): Promise<Account> {
    return this.register({
      bank: {
        SharedBank: null,
      },
      name: [],
    });
  }

  async register(input: RegisterAccountInput): Promise<Account> {
    const account_details = await this.actor.register_account(input);

    if ('Err' in account_details) {
      throw account_details.Err;
    }

    return account_details.Ok.account;
  }

  async editAccount(input: ManageAccountInput): Promise<AccountDetails> {
    const account_details = await this.actor.manage_account(input);

    if ('Err' in account_details) {
      throw account_details.Err;
    }

    return account_details.Ok.account_details;
  }
}
