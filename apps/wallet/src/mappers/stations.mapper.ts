import { Principal } from '@dfinity/principal';
import { UserStation } from '~/generated/control-panel/control_panel.did';
import { Station } from '~/stores/session.store';

export function stationToUserStation(station: Omit<Station, 'main'>): UserStation {
  return {
    canister_id: Principal.fromText(station.canisterId),
    name: station.name ? [station.name] : [],
  };
}
