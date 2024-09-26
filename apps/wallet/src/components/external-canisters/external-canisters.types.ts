import { Principal } from '@dfinity/principal';
import { CanisterInstallMode } from '~/generated/station/station.did';

export interface CanisterTopUpModel {
  canisterId?: Principal;
  cycles?: bigint;
}

export interface CanisterIcSettingsModel {
  canisterId?: Principal;
  freezing_threshold?: number;
  controllers?: Principal[];
  memory_allocation?: number;
  compute_allocation?: number;
  reserved_cycles_limit?: number;
}

export interface CanisterInstallModel {
  canisterId?: Principal;
  wasmModule?: Uint8Array;
  wasmInstallArg?: Uint8Array;
  mode?: CanisterInstallMode;
}
