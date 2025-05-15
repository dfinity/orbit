import { spawnSync } from 'child_process';
import { stdout } from 'process';

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
