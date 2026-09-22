import { spawnSync } from 'child_process';

export function getCanisterInfo(canisterId: string): {
  stdout: string;
  moduleHash: string | null;
} {
  const result = spawnSync('dfx', ['canister', 'info', canisterId]);

  const stdout = result.stdout.toString();
  // Parse module hash from the output
  const moduleHashMatch = stdout.match(/Module hash: (0x[a-f0-9]+)/);
  const moduleHash = moduleHashMatch ? moduleHashMatch[1] : null;

  return {
    stdout,
    moduleHash,
  };
}

/**
 * Returns the raw `health_status` reply of a station, e.g. `(variant { Healthy })`.
 *
 * The call fails while the station is stopped or has no module installed, in which case the
 * returned output is empty.
 */
export function getStationHealthStatus(stationId: string): string {
  const result = spawnSync('dfx', ['canister', 'call', stationId, 'health_status']);

  return result.stdout?.toString() ?? '';
}

/**
 * Returns the `app:version` metadata of a canister, e.g. `0.7.1` for a station.
 */
export function getCanisterAppVersion(canisterId: string): string {
  const result = spawnSync('dfx', ['canister', 'metadata', canisterId, 'app:version']);

  return result.stdout.toString().trim();
}
