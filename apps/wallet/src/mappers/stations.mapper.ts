import { Principal } from '@icp-sdk/core/principal';
import { UserStation } from '~/generated/control-panel/control_panel.did';
import { StoreUserStation } from '~/stores/session.store';

export function storeUserStationToUserStation(station: StoreUserStation): UserStation {
  return {
    canister_id: Principal.fromText(station.canisterId),
    name: station.name,
    labels: station.labels,
  };
}

export function userStationToStoreUserStation(station: UserStation): StoreUserStation {
  return {
    canisterId: station.canister_id.toText(),
    name: station.name,
    labels: station.labels,
  };
}
