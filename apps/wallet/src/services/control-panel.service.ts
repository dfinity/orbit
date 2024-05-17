import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { appInitConfig } from '~/configs/init.config';
import { idlFactory } from '~/generated/control-panel';
import {
  CanDeployStationResponse,
  DeployStationInput,
  ListUserStationsInput,
  ManageUserStationsInput,
  RegisterUserInput,
  User,
  UserStation,
  _SERVICE,
} from '~/generated/control-panel/control_panel.did';
import { transformIdlWithOnlyVerifiedCalls, variantIs } from '~/utils/helper.utils';

export class ControlPanelService {
  // This actor is modified to only perform calls that can be verified, such as update calls that go through consensus.
  private verified_actor: ActorSubclass<_SERVICE>;

  // This is the default actor that is used to perform all calls, including query calls.
  private actor: ActorSubclass<_SERVICE>;

  constructor(agent: HttpAgent) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent,
      canisterId: appInitConfig.canisters.controlPanel,
    });

    this.verified_actor = Actor.createActor<_SERVICE>(
      transformIdlWithOnlyVerifiedCalls(idlFactory),
      {
        agent,
        canisterId: appInitConfig.canisters.controlPanel,
      },
    );
  }

  async getCurrentUser(verifiedCall = false): Promise<User> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_user();
    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async subscribeToWaitlist(email: string): Promise<void> {
    const result = await this.actor.subscribe_to_waiting_list(email);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }
  }

  async hasRegistration(verifiedCall = false): Promise<boolean> {
    return await this.getCurrentUser(verifiedCall)
      .then(_ => true)
      .catch(() => false);
  }

  async register(input: RegisterUserInput): Promise<User> {
    const result = await this.actor.register_user(input);

    if (variantIs(result, 'Err')) {
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

  async manageUserStations(input: ManageUserStationsInput): Promise<void> {
    const result = await this.actor.manage_user_stations(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }
  }

  async listUserStations(
    input: ListUserStationsInput,
    verifiedCall = false,
  ): Promise<UserStation[]> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_user_stations(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.stations;
  }

  async deployStation(input: DeployStationInput): Promise<Principal> {
    const result = await this.actor.deploy_station(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.canister_id;
  }

  async canDeployStation(verifiedCall = false): Promise<CanDeployStationResponse> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.can_deploy_station();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }
}
