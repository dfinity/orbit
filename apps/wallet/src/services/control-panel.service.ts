import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { appInitConfig } from '~/configs/init.config';
import { idlFactory } from '~/generated/control-panel';
import {
  CanDeployStationResponse,
  DeployStationInput,
  GetArtifactInput,
  GetArtifactResult,
  GetRegistryEntryInput,
  GetRegistryEntryResult,
  ListUserStationsInput,
  ManageUserStationsInput,
  RegisterUserInput,
  RegistryEntry,
  SearchRegistryInput,
  SearchRegistryResult,
  User,
  UserStation,
  _SERVICE,
} from '~/generated/control-panel/control_panel.did';
import { ExtractOk } from '~/types/helper.types';
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

  async findNextModuleVersion(
    input: {
      name: string;
      currentVersion: string;
    },
    verifiedCall = false,
  ): Promise<RegistryEntry | null> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.next_wasm_module_version({
      name: input.name,
      current_version: input.currentVersion,
    });

    if (variantIs(result, 'Err')) {
      switch (result.Err?.code) {
        case 'WASM_MODULE_NOT_FOUND':
          // If the module is not found then there is no next version.
          return null;
        default:
          throw result.Err;
      }
    }

    return result.Ok.entry?.[0] ? result.Ok.entry[0] : null;
  }

  async findRegistryEntries(
    input: SearchRegistryInput,
    verifiedCall = false,
  ): Promise<ExtractOk<SearchRegistryResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.search_registry(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getRegistryEntry(
    input: GetRegistryEntryInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetRegistryEntryResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_registry_entry(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async findModuleVersionRegistryEntry(
    name: string,
    version: string,
  ): Promise<RegistryEntry | null> {
    let next_offset: bigint | null = BigInt(0);
    do {
      const result = await this.findRegistryEntries({
        filter_by: [{ Name: name }],
        pagination: [{ offset: [next_offset!], limit: [100] }],
        sort_by: [{ Version: { Desc: null } }],
      });

      if (result.entries.length === 0) {
        return null;
      }

      for (const entry of result.entries) {
        if (variantIs(entry.value, 'WasmModule') && entry.value.WasmModule.version === version) {
          // If the version is found we fetch it again with an update call to ensure we get verified data.
          return (await this.getRegistryEntry({ id: entry.id }, true)).entry;
        }
      }

      next_offset = result.next_offset?.[0] ? result.next_offset[0] : null;
    } while (next_offset !== null);

    return null;
  }

  async getArtifact(
    input: GetArtifactInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetArtifactResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_artifact(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }
}
