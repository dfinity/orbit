import { Principal } from '@dfinity/principal';

export interface CanisterTopUpModel {
  canisterId?: Principal;
  cycles?: bigint;
}
